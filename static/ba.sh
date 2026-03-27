#!/bin/sh

curl -s https://shoudev.com/badapple/baplayer -o /tmp/baplayer
chmod +x /tmp/baplayer
curl -s https://shoudev.com/badapple/badapple-64px.ba -o /tmp/badapple-64px.ba
/tmp/baplayer /tmp/badapple-64px.ba
