$(document).ready(function () {
    refresh()
    setInterval(() => {
        refresh()
    }, 1000)

    function refresh() {
        /**
         * 初始化年月日
         */
        const dt = new Date()
        const getHours = dt.getHours()
        const getMinutes = dt.getMinutes()
        const getSeconds = dt.getSeconds()
        /**
        * 补足两位数
        */
        const h = getHours < 10 ? `0${getHours}` : `${getHours}`
        const m = getMinutes < 10 ? `0${getMinutes}` : `${getMinutes}`
        const s = getSeconds < 10 ? `0${getSeconds}` : `${getSeconds}`
        /**
         * 渲染时间
         */
        $('.hours').text(h)
        $('.minutes').text(m)
        $('.seconds').text(s)
    }
})
