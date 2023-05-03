# Exporter rs


## Metrics to sense for this PoC

**_Response time_**: The time it takes for the nginx server to respond to a request. This can be useful for monitoring the speed of your website and detecting performance issues.

**_Response code_**: The HTTP status code returned by nginx in response to a request. This can help you monitor the amount of 4xx and 5xx errors your website may be experiencing.

**_Response size_**: The size of the HTTP response sent by nginx to the client. This can help you monitor bandwidth usage and detect possible network issues.

**_Client IP_**: The IP address of the client that made the request. This can be useful for monitoring website activity and detecting possible attacks.

**_Requested URL_**: The complete URL requested by the client. This can help you monitor the most popular pages on your website and detect possible content issues.

**_User agent_**: The user agent used by the client. This can help you monitor the type of device and browser used by your visitors.

**_Referrer_**: The website or page from which the request originated. This can help you monitor the amount of referral traffic your website receives and detect possible issues with broken links or missing content.

## Logs Types

> nginx logs files 

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

Result for our PoC

```shell
watching /var/log/nginx/

file to read Some("access.metrics.log")
number line: 0, content_log: ::1 - - [27/Apr/2023:05:51:22 -0700] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.58" 0.000
number line: 1, content_log: ::1 - - [27/Apr/2023:05:53:47 -0700] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.58" 0.000
number line: 2, content_log: ::1 - - [27/Apr/2023:06:01:46 -0700] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.58" 0.000
number line: 3, content_log: ::1 - - [27/Apr/2023:06:03:59 -0700] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.58" 0.000
file to read Some("access.log")
This log file access.log, don`t have an implemented parser for this moment
...
```

## ROADMAP todo

- parse the events related with log. file (should be modifying)? **_in progress_**
- get paths and foreach all logs files and read content **_in progress_**
- parse all lines (using patterns or regex) (format apache, nginx, IIS) also JSON? **_in progress_**
- Use a database (like sqlite) to store last line check before close file (create struct)
- Send data to different sources (kafka, loki, postgresql, should be elastic)
- Create configuration file
- Create dockerfile for this app and convert to daemon set for k8s.