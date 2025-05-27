FROM searxng/searxng:2025.3.31-08885d061

RUN mkdir -p /etc/searxng

RUN apk upgrade && apk add uwsgi-router_basicauth

COPY settings.yml /etc/searxng/settings.yml
COPY uwsgi.ini /etc/searxng/uwsgi.ini
COPY auth-file /etc/searxng/auth-file

ENV INSTANCE_NAME=searxng \
    SEARXNG_SETTINGS_PATH=/etc/searxng/settings.yml \
    UWSGI_SETTINGS_PATH=/etc/searxng/uwsgi.ini

CMD ["python3", "-m", "searxng"]
