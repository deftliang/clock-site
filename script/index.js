$(document).ready(function () {
    refresh()
    setInterval(() => {
        refresh()
    }, 1000)

    function refresh() {
        let dt = new Date()
        let h = dt.getHours() < 10 ? `0${dt.getHours()}` : `${dt.getHours()}`
        let m = dt.getMinutes() < 10 ? `0${dt.getMinutes()}` : `${dt.getMinutes()}`
        let s = dt.getSeconds() < 10 ? `0${dt.getSeconds()}` : `${dt.getSeconds()}`
        // console.log(dt, h, m, s)
        $('.hours').text(h)
        $('.minutes').text(m)
        $('.seconds').text(s)
    }
})
