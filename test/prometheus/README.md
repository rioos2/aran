# prometheus stub testing

This is the stub testing setup for command center with two nodes.

- `http://localhost:7080/metrics`
- `http://localhost:7081/metrics`

## Pre requistes

Download and untar  [prometheus 2.0 beta5 > ](https://github.com/prometheus/prometheus/releases/download/v2.0.0-beta.5/prometheus-2.0.0-beta.5.linux-amd64.tar.gz)

###  Build the main.go with binary named `node2`

```

cd test/prometheus/node1

go get -d

go build


```

###  Build the main.go with port 7081 with binary named `node2`

```

cd test/prometheus/node2

go get -d

go build


```

### Edit `$HOME/software/prometheus/prometheus.yaml`.


```

cd $HOME/software/prometheus

nano prometheus.yaml

```

Replace the **scrape_configs** section with the below

```

scrape_configs:
  - job_name: 'prometheus'

    # Override the global default and scrape targets from this job every 5 seconds.
    scrape_interval: 5s

    static_configs:
      - targets: ['localhost:9090']

  - job_name:       'commandcenter-stub'

    # Override the global default and scrape targets from this job every 5 seconds.
    scrape_interval: 5s

    static_configs:
      - targets: ['localhost:7080']
        labels:
          node: 'firstcompute.riocorp.io'
          group: 'canary'

      - targets: ['localhost:7081']
        labels:
          node: 'second.riocorp.io'
          group: 'canary'

```

### Start node1. node2

In a terminal do,

```

cd test/prometheus/node1

./node1

```
In another terminal do,

```

cd test/prometheus/node2

./node2

```
### Start prometheus

Lets us say that you have installed prometheus in `$HOME/software/prometheus`

```

cd $HOME/software/prometheus

./prometheus

```

This will pickup the new `prometheus.yaml` in `$HOME/software/prometheus`.

### testing

```

curl --header "X-AUTH-RIOOS-EMAIL:paul@riocorp.io" --header "Authorization:Bearer 1fJI78LW4jqBsZ6oXK"  http://localhost:9636/api/v1/healthz/overall

```
