# Telegram article storage bot

## Usage
Set token API in `TELOXIDE_TOKEN` variable

## Run on server
```
export TELOXIDE_TOKEN=YOUR_TOKEN ; nohup bash ci-cd.sh > /dev/null 2>&1 &
```

## TODO:
- [ ] Add migration from Pocket
- [ ] Add article parsing and saving using soup crate
- [ ] Limit files uploading
- [ ] Add statistics
- [ ] Add "Mark as read" button under an article message *unable to implement due to no library support, waiting for the feature
