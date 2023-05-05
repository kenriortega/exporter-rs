# Exporter rs


## Metrics to sense for this PoC

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
```Nginx
        ...
        ##
        # Logging Settings
        ##

          log_format metrics '$remote_addr - $remote_user [$time_local] '
                      '"$request" $status $body_bytes_sent '
                      '"$http_referer" "$http_user_agent" '
                      '$request_time';

        access_log /var/log/Nginx/access.metrics.log metrics;
        ....
```

Result for our PoC

```shell
watching /var/log/Nginx/

file to read Some("access.metrics.log")
last offset from txt : 10
offset > 0 : 10
kafka: {"remote_addr":"::1","time_local":"05/May/2023:14:24:29 -0700","request":"GET / HTTP/1.1","status":200,"body_bytes_sent":612,"http_referer":"-","http_user_agent":"Mozilla/5.0 (Windows NT; Windows NT 10.0; en-US) WindowsPowerShell/5.1.22621.963","request_time":0.0}
postgres: {"remote_addr":"::1","time_local":"05/May/2023:14:24:29 -0700","request":"GET / HTTP/1.1","status":200,"body_bytes_sent":612,"http_referer":"-","http_user_agent":"Mozilla/5.0 (Windows NT; Windows NT 10.0; en-US) WindowsPowerShell/5.1.22621.963","request_time":0.0}
last offset to commit 11
file to read Some("access.log")

...
```

## ROADMAP todo

- Create configuration file **done**
- get paths and foreach all logs files and read content **_done_**
- parse the events related with log. file (should be modifying)? **_in progress_**
- parse all lines (using patterns or regex) (format apache, Nginx, IIS) also JSON? **_in progress_**
- Use a database (like sqlite) to store last line check before close file (create struct, offset file) **_in progress_**
- Send data to different sources (kafka, loki, postgresql, should be elastic)
- Parse time_local to unix timestamp
