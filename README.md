# HostCat
UNIX Command Line tool to switch between Local Dns profiles

HostCat allow users to create multiple profiles using which they can quickly switch between local domain names for ip 127.0.0.1 in `/etc/hosts`

For Example these two profiles

```
foo -> foo.com api.foo.com
bar -> bar.com api.bar.com
```

allow users to create local DNS for `foo.com api.foo.com` and `bar.com api.bar.com` and assign a profile to them, which can be switched using hostcat effortlessly

## Creating a profile

*create a plain config file (required once)*
```shell script
$ sudo sh -c "echo \"default localhost\" >> /etc/hostcat"
```

*create a profile*
```shell script
$ sudo hostcat set -p foo -v "foo.com api.foo.com"
```
Here `-p` donates a profile name and `-v` donates its' Dns

## Switch profile

```shell script
$ sudo hostcat switch -p foo
```

## Print Profiles

```shell script
$ sudo hostcat profiles
```