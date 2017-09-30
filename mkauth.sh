#!/bin/bash

if [ -f auth.conf ]; then
	echo 'auth.conf exists in current directory, delete it first'
	exit 1
fi

echo 'User account or bot? ("user", "bot")'
read uob

case $uob in

user)
	echo 'export DI_AUTH_MODE=user' >> auth.conf
	echo 'Email:'
	read email
	echo "export DI_EMAIL=$email" >> auth.conf
	echo 'Password:'
	read pass
	echo "export DI_PASSWORD=$pass" >> auth.conf
	;;

bot)
	echo 'export DI_AUTH_MODE=bot' >> auth.conf
	echo 'Bot token:'
	read token
	echo "export DI_BOT_TOKEN=$token" >> auth.conf
	;;

esac

echo 'Written to auth.conf'
