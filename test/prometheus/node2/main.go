// Copyright 2017 RioCorp Inc

// A simple example exposing fictional CPU/RAM/Disk latencies as Prometheus
// metrics.
package main

import (
	"flag"
	"log"
	"fmt"
	//"math"
	"math/rand"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	addr              = flag.String("listen-address", ":7081", "The address to listen on for HTTP requests.")
	uniformDomain     = flag.Float64("uniform.domain", 0.2, "The domain for the uniform distribution.")
	oscillationPeriod = flag.Duration("oscillation-period", 10*time.Minute, "The duration of the rate oscillation period.")
)

var (

	cpuUsedCapacity = prometheus.NewGauge(prometheus.GaugeOpts{
		Name: "cpu_total",
		Help: "Current cpu used capacity of node.",
	})

	ramUsedCapacity = prometheus.NewGauge(prometheus.GaugeOpts{
		Name: "ram_total",
		Help: "Current ram used capacity of node.",
	})

	diskUsedCapacity = prometheus.NewGauge(prometheus.GaugeOpts{
		Name: "disk_total",
		Help: "Current disk capacity of node.",
	})
)

func init() {
	prometheus.MustRegister(cpuUsedCapacity)
	prometheus.MustRegister(ramUsedCapacity)
	prometheus.MustRegister(diskUsedCapacity)
}

func main() {
	flag.Parse()

	//start := time.Now()

	/*oscillationFactor := func() float64 {
		return 2 + math.Sin(math.Sin(2*math.Pi*float64(time.Since(start))/float64(*oscillationPeriod)))
	}*/

	go func() {
		for {
			v := (rand.Float64() * *uniformDomain)*100
			fmt.Printf("cpu =>%f, ram =>%f, disk =>%f\n", v, (v+5), (v+10))
			cpuUsedCapacity.Set(v)
			ramUsedCapacity.Set(v+5)
			diskUsedCapacity.Set(v+10)
			time.Sleep(2 * time.Second)
		}
	}()


	// Expose the registered metrics via HTTP.
	http.Handle("/metrics", promhttp.Handler())
	fmt.Printf("Node listening: %s/%s\n", "http://127.0.0.1:7081", "metrics")
	log.Fatal(http.ListenAndServe(*addr, nil))
}
