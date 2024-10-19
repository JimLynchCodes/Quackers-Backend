# Provisioning A New Server [on Vultr]


# 1) Create an SSH Key
This will be used to access the server.

To generate:
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
```

Then copy the public key:
```bash
cat /Users/jim/.ssh/id_ed25519.pub | pbcopy
```

Paste the text as a new key in the [Vultr SSH Keys section](https://my.vultr.com/settings/#settingssshkeys).

<br/>

# 2) Purchase Server ($)
On [Vultr deploy](https://my.vultr.com/deploy/) section, get a new server.

I am choosing these settings:
- Cloud Compute - Shared CPU
- New York (NJ)
- Ubuntu 24.10 x64
- AMD High Performance, 25 GB NMVe 1 vCPU Core, 1GB Memory, 25 GB NVMe Storage, 2TB Bandwidth, $6/month
- Disabled auto-backups
- IPv6 free
- Choose your ssh key from step 1
- give it any server name like: quackers-beta-1


_Note the public ip address once your compute instance is running!_

</br>

# 3) Set Domain Records (point domain to server)
In order to use SSL you need to buy a domain from somewhere.

Go to domain settings and Edit DNS settings for your domain (eg. evaluates2.com):
 
 Under Subdomain Records click `+ Add Record`

Enter this in the fields:

- Subdomain: quackers-beta
- Record Type: A
- IP Address or Target Host: your server ip from step 2

Then save the record and give it a few min / hours to propagate.

To check if it actually propagated using DNS Checker:
Visit [DNS Checker](https://dnschecker.org/#A/www.quackers-beta.jimlynchcodes.com) and change it to your domain.

Enter Your Domain: In the search box, type the domain you want to check (e.g., quackers-beta.jimlynchcodes.com).

Select Record Type: Choose the type of DNS record you want to check (e.g., A, CNAME, MX, etc.) from the dropdown menu.

Click Search: Hit the Search button.

View Results: The tool will display results showing whether your domain resolves correctly from different locations around the world.

<br/>

## 4)  Set ports & firewall stuff

First, let's ssh into the server:
```bash
ssh -i /Users/jim/.ssh/id_ed25519 root@[ip address]
```

```bash
sudo apt install ufw
```

```bash
sudo ufw allow OpenSSH
```
```bash
sudo ufw allow 443/tcp 
```

```bash
sudo ufw enable
```

Then `exit` to quit the ssh session.

<br/>

## 5) Move SystemD Config File
move quackers_ws_server_systemd.service to /etc/systemd/system/quackers_ws_server_systemd.service on the server

```bash
scp -r ./quackers_ws_server_systemd.service root@{{ secrets.SERVER_IP_ADDRESS }}:/etc/systemd/system/quackers_ws_server_systemd.service
```

</br>

## 6 ) Move Nginx File
```bash
scp -r ./quackers_ws_ngix_config root@xxx.xxx.xx.xxx:/etc/nginx/sites-available/quackers-beta.jimlynchcodes.com
```

Might need to delete (or comment out) the `/etc/nginx/sites-available/default` file.

Might need to point it at the new config file:
```bash
sudo nginx -t -c /etc/nginx/sites-available/quackers-beta.jimlynchcodes.com
```

</br>

## 7) Move Compiled Executable

First, make a live folder under root:
```bash
mkdir /root/live
```

Then move executable there:

```bash
scp -r ./target/x86_64-unknown-linux-gnu/release/quackers-ws-server root@xxx.xxx.xx.xxx:/root/live/
```

</br>

## 8) Check Nginx

4c) Test new configuration: `sudo nginx -t`

4e) Reload Nginx: `sudo nginx -s reload` or `sudo systemctl restart nginx`

nginx logs:
```bash
sudo tail -f /var/log/nginx/error.log
```

or 
```bash
sudo tail -f /var/log/nginx/access.log
```

<br/>

## ) check firewall ports
```bash
sudo ufw status verbose
```

## 9) SSL SetUp (Let's Encrypt Certbot ssl with domain)

Install Certbot for Let's Encrypt on your Ubuntu server:
```bash
sudo apt install certbot python3-certbot-nginx
```

Obtain an SSL certificate:
_(Note the usage of the dot for subdomain)_
```bash
sudo certbot --nginx -d quackers-beta.evaluates2.com -d www.quackers-beta.evaluates2.com
```

Follow the prompts to complete the SSL certificate installation.

Test automatic renewal:
```bash
sudo certbot renew --dry-run
```

## 10) Start Up Server Running!
- run enable (to start service automatically on server bootup)
```bash
sudo systemctl enable /etc/systemd/system/quackers_ws_server_systemd.service
```

- run start (to start service running right now)
```bash
sudo systemctl start /etc/systemd/system/quackers_ws_server_systemd.service
```

- or restart
```bash
sudo systemctl restart quackers_ws_server_systemd.service
```

- see what's running:
```bash
systemctl list-units --type=service
```

- see run logs (useful if it can't start)
```bash
sudo journalctl -u quackers_ws_server_systemd.service --no-pager
```

- reload
```bash
sudo systemctl daemon-reload
```

## 11) Update Github Secrets
Change the secrets `SERVER_SSH_KEY` and `SERVER_IP_ADDRESS` on Github for this repository so that it can be automatically redeployed when new git tags are pushed. 👍 🚀
