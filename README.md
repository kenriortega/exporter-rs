# Exporter rs


## Metcis to sense for this PoC

**_Response time_**: The time it takes for the Nginx server to respond to a request. This can be useful for monitoring the speed of your website and detecting performance issues.

**_Response code_**: The HTTP status code returned by Nginx in response to a request. This can help you monitor the amount of 4xx and 5xx errors your website may be experiencing.

**_Response size_**: The size of the HTTP response sent by Nginx to the client. This can help you monitor bandwidth usage and detect possible network issues.

**_Client IP_**: The IP address of the client that made the request. This can be useful for monitoring website activity and detecting possible attacks.

**_Requested URL_**: The complete URL requested by the client. This can help you monitor the most popular pages on your website and detect possible content issues.

**_User agent_**: The user agent used by the client. This can help you monitor the type of device and browser used by your visitors.

**_Referrer_**: The website or page from which the request originated. This can help you monitor the amount of referral traffic your website receives and detect possible issues with broken links or missing content.

## Logs Types

> Nginx logs files 

Add this line before to use this logger exporter.
```nginx
        ...
        ##
        # Logging Settings
        ##

          log_format metrics '$remote_addr - $remote_user [$time_local] '
                      '"$request" $status $body_bytes_sent '
                      '"$http_referer" "$http_user_agent" '
                      '$request_time';

        access_log /var/log/nginx/access.metrics.log metrics;
        ....
```

## ROADMAP todo

- parse the events related with log.file (should be Modify)? **_in progress_**
- get paths and foreach all logs files and read content **_in progress_**
- parse all lines (using patterns or regex) (format apache, nginx, IIS) also JSON? **_in progress_**
- Use a database (like sqlite) to store last line check before close file (create struct)
- Send data to different sources (kafka, loki, postgresql, should be elastic)
- Create configuration file
- Create dockerfile for this app and convert to daemon set for k8s.