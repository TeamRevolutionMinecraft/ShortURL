# URL Shortener

[Handbuch](docs/DE_de_Portainer.md)

Configuration Options:
```
# Format
NAME=VALUE

# Redirects /name to destination url
REDIRECT_<NAME>=<destination url>

# Redirects / and /index to destination url
REDIRECT_INDEX=<destination url>

# Redirects any invalid request to destination url
NOT_FOUND_REDIRECT=<destination url>
```