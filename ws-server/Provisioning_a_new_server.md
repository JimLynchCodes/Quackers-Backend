# Provisioning A New Server

# 1) Purchase Server ($$$)
You can get a regular ubuntu server from digital ocean, aws, google cloud, azure, cloudflare, etc...

Find two key pieces of information: the ___public ip address___ and the ___ssh key___.


# 2) Set Domain Records (point domain to server)
In order to use SSL you need to buy a domain from somewhere. I'll be using: _.com 

Go to domain settings and add an A Record:

 - Host: subdomain (e.g., if your subdomain is quackers-beta, enter quackers-beta).
 
 - Record Type: A
 
 - Value: <your_server_ip_address> (replace this with the public IP address of your DigitalOcean server).
 
 - TTL: You can leave it as default or set it to 300 seconds.

Then save the record and give it a few min / hours to propagate.


## 3) SSL SetUp (Let's Encrypt Certbot ssl with domain)

Install Certbot for Let's Encrypt on your Ubuntu server:
```bash
sudo apt install certbot python3-certbot-nginx
```

Obtain an SSL certificate:
```bash
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com
```

Follow the prompts to complete the SSL certificate installation.


## 4) Nginx Setup (ports and firewalls configuration)

Follow commands in the `quackers_ws_ngix_config` and move the file to the location it describes.


## 5) SystemD Setup (Running and auto-restarting the app)

move quackers_ws_server_systemd.service to /etc/systemd/system/quackers_ws_server_systemd.service on the server
```
scp -r ./ws-server/quackers_ws_server_systemd.service root@${{ secrets.SERVER_IP_ADDRESS }}:/etc/systemd/system/quackers_ws_server_systemd.service
```

(Run these commands on server)

_// Note: you can also just push a git tag mentioning beta to kick off these later parts

- run enable (to start service automatically on server bootup)
```bash
sudo systemctl enable /etc/systemd/system/quackers_ws_server_systemd.service
```

- run start (to start service running right now)
```bash
sudo systemctl start /etc/systemd/system/quackers_ws_server_systemd.service
```
