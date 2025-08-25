#!/usr/bin/env python3

import os
import re
import urllib.request
import urllib.error
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime
import threading
import time
import random

# Thread-safe counters
alive_count = 0
dead_count = 0
count_lock = threading.Lock()

def find_markdown_files():
    """Find all markdown files in the current directory"""
    md_files = []
    docs_path = "."
    
    if not os.path.exists(docs_path):
        print(f"Current folder not found")
        return md_files
    
    for root, dirs, files in os.walk(docs_path):
        for file in files:
            if file.endswith('.md'):
                md_files.append(os.path.join(root, file))
    
    return md_files

def extract_links_from_file(file_path):
    """Extract HTTP/HTTPS links from a markdown file"""
    links = []
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, 1):
                # Find markdown links [text](url)
                markdown_links = re.findall(r'\[([^\]]*)\]\(([^)]+)\)', line)
                for text, url in markdown_links:
                    if url.startswith(('http://', 'https://')):
                        links.append((file_path, line_num, url.strip()))
                
                # Find direct links <url>
                direct_links = re.findall(r'<(https?://[^>]+)>', line)
                for url in direct_links:
                    links.append((file_path, line_num, url.strip()))
    
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
    
    return links

def check_link_with_retry(file_path, line_num, url, max_retries=3):
    """Check if a link is alive or dead with retry logic for rate limiting"""
    global alive_count, dead_count
    
    for attempt in range(max_retries):
        try:
            # Create request with a user agent to avoid 403 errors
            req = urllib.request.Request(
                url,
                headers={'User-Agent': 'Mozilla/5.0 (compatible; LinkChecker/1.0)'}
            )
            
            # Set timeout for the request
            with urllib.request.urlopen(req, timeout=15) as response:
                status_code = response.getcode()
                if 200 <= status_code < 400:
                    with count_lock:
                        alive_count += 1
                    return 'ALIVE', file_path, line_num, url, None
                else:
                    with count_lock:
                        dead_count += 1
                    return 'DEAD', file_path, line_num, url, f"HTTP {status_code}"
        
        except urllib.error.HTTPError as e:
            if e.code == 429:  # Too Many Requests
                if attempt < max_retries - 1:  # Don't wait on last attempt
                    # Exponential backoff with jitter
                    wait_time = (2 ** attempt) + random.uniform(0, 1)
                    print(f"⏳ Rate limited for {url}, waiting {wait_time:.1f}s before retry {attempt + 2}/{max_retries}")
                    time.sleep(wait_time)
                    continue
                else:
                    # Final attempt failed with rate limiting
                    with count_lock:
                        alive_count += 1  # Assume it's alive if we're just rate limited
                    return 'ALIVE', file_path, line_num, url, "Rate limited (assumed alive)"
            else:
                # Other HTTP errors (404, 403, etc.) - don't retry
                with count_lock:
                    dead_count += 1
                return 'DEAD', file_path, line_num, url, f"HTTP {e.code}: {e.reason}"
        
        except (urllib.error.URLError, Exception) as e:
            if attempt < max_retries - 1:
                # Retry on network errors
                wait_time = (2 ** attempt) + random.uniform(0, 1)
                print(f"⏳ Network error for {url}, waiting {wait_time:.1f}s before retry {attempt + 2}/{max_retries}")
                time.sleep(wait_time)
                continue
            else:
                # Final attempt failed
                with count_lock:
                    dead_count += 1
                return 'DEAD', file_path, line_num, url, str(e)

def main():
    global alive_count, dead_count
    
    print("Starting markdown link checking...")
    print("==================================")
    
    # Find all markdown files
    print("Finding markdown files in current folder and extracting links...")
    md_files = find_markdown_files()
    
    if not md_files:
        print("No markdown files found in current folder.")
        return
    
    # Extract all links
    all_links = []
    for file_path in md_files:
        links = extract_links_from_file(file_path)
        all_links.extend(links)
    
    print(f"Found {len(all_links)} HTTP/HTTPS links to check in current folder")
    
    if not all_links:
        print("No HTTP/HTTPS links found in markdown files.")
        return
    
    # Check links in parallel
    print("Checking links...")
    dead_links = []
    results = []
    
    # Use ThreadPoolExecutor for parallel processing
    with ThreadPoolExecutor(max_workers=20) as executor:
        # Submit all link checking tasks
        future_to_link = {
            executor.submit(check_link_with_retry, file_path, line_num, url): (file_path, line_num, url)
            for file_path, line_num, url in all_links
        }
        
        # Process results as they complete
        for future in as_completed(future_to_link):
            status, file_path, line_num, url, error = future.result()
            
            if status == 'ALIVE':
                if error:
                    print(f"✓ ALIVE: {url} ({error})")
                else:
                    print(f"✓ ALIVE: {url}")
            else:
                print(f"✗ DEAD: {url} ({error})")
                dead_links.append((file_path, line_num, url, error))
            
            results.append((status, file_path, line_num, url, error))
    
    # Calculate statistics
    total_checked = alive_count + dead_count
    alive_percentage = (alive_count * 100) // total_checked if total_checked > 0 else 0
    dead_percentage = (dead_count * 100) // total_checked if total_checked > 0 else 0
    
    # Display summary
    print("\n===============================")
    print("LINK CHECK SUMMARY")
    print("===============================")
    print(f"Total links found: {len(all_links)}")
    print(f"Total links checked: {total_checked}")
    print(f"Alive links: {alive_count} ({alive_percentage}%)")
    print(f"Dead links: {dead_count} ({dead_percentage}%)")
    print()
    
    # Save dead links to file and display them
    if dead_links:
        output_file = "dead_links_report.txt"
        with open(output_file, 'w') as f:
            f.write(f"Dead Links Report - {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
            f.write("================================\n\n")
            
            for file_path, line_num, url, error in dead_links:
                f.write(f"File: {file_path}\n")
                f.write(f"Line: {line_num}\n")
                f.write(f"URL: {url}\n")
                f.write(f"Error: {error}\n")
                f.write("---\n")
        
        print(f"Dead links details saved to: {output_file}")
        print("\nDEAD LINKS:")
        print("----------")
        for file_path, line_num, url, error in dead_links:
            print(f"{file_path}:{line_num} - {url}")
    else:
        print("🎉 All links are alive!")
    
    print("\nLink checking complete!")

if __name__ == "__main__":
    main()
