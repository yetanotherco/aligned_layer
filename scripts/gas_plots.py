import matplotlib.pyplot as plt
import numpy as np
from scipy.stats import linregress

# Original data
### For merkle Tree
N = np.array([2, 4, 8, 16, 32, 64, 128, 256, 512, 1024])
gasUsed = np.array([28323, 30155, 33692, 40641, 54422, 81896, 136874, 247349, 473752, 936605])
# N = np.array([2, 4, 8, 16, 32, 64, 128])
# gasUsed = np.array([28323, 30155, 33692, 40641, 54422, 81896, 136874])
### Per signature
# N = np.array([1, 2, 4])
# gasUsed = np.array([33893, 39351, 50256])

# Perform linear regression
slope, intercept, r_value, p_value, std_err = linregress(N, gasUsed)

# Generate predicted gasUsed values for the regression line
gasUsed_pred = intercept + slope * N

# Print the slope and intercept in y = mx + b
print(f'y = {slope}x + {intercept}')
print(f'R-squared: {r_value**2}')
print(f'p-value: {p_value}')
print(f'Standard Error: {std_err}')

# Calculate price per N
price_per_N = gasUsed_pred / N

# Plotting the linear regression
plt.figure(figsize=(10, 6))
plt.scatter(N, gasUsed, color='blue', marker='o', label='Original Data')
plt.plot(N, gasUsed_pred, color='red', label='Linear Regression')
plt.xlabel('Batch size')
plt.ylabel('Gas Used')
plt.title('Gas Used vs. Batch size')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.show()

# Plotting price per N
plt.figure(figsize=(10, 6))
plt.plot(N, price_per_N, color='green', label='Price per proof')
plt.xlabel('Batch size')
plt.ylabel('Price per proof')
plt.title('Price per proof vs. Batch size')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.show()
