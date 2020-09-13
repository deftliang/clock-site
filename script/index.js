$(document).ready(function () {
    refresh()
    setInterval(() => {
        refresh()
    }, 1000)

    function refresh() {
        const dt = new Date()
        const getHours = dt.getHours()
        const getMinutes = dt.getMinutes()
        const getSeconds = dt.getSeconds()
        const h = getHours < 10 ? `0${getHours}` : `${getHours}`
        const m = getMinutes < 10 ? `0${getMinutes}` : `${getMinutes}`
        const s = getSeconds < 10 ? `0${getSeconds}` : `${getSeconds}`
        $('.hours').text(h)
        $('.minutes').text(m)
        $('.seconds').text(s)
    }
})
