# Function to send telegram message
# @param message
curl -s -X POST https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/sendMessage \
    -d chat_id=$TELEGRAM_CHAT_ID \
    -d text="$1" \
    -d disable_notification=true
    
