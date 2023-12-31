
# ENTSOE Transparency API Client

## Environment Variables

| Variable | Description | Default |
| --- | --- | --- |
| ENTSOE_API_KEY | API key for ENTSOE API | |
| ENTSOE_LOG_LEVEL | Log level for the application | INFO |

## Docker

```shell
docker run -v $(pwd)/.env:/app/.env entsoe-rs 
```

## Test forecast endpoint with

```shell
curl '127.0.0.1:3000/forecast?area_code=DE_50HZ&process_type=A01&document_type=A69&psr_type=B16&period_start=2015-12-31T23:00:00Z&period_end=2016-01-04T23:30:00Z&in_domain=DE_50HZ'
```

## Swagger and Redoc

Available at paths `/swagger-ui` and `/redoc` respectively.
