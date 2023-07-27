FROM nginx as web
COPY ./nginx.conf /etc/nginx/nginx.conf
