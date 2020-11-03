$(document).ready(function () {
    /**
     * 显示模式开关
     */
    let flag = false

    /**
     * 时钟运行
     */
    refresh()
    setInterval(() => {
        refresh()
    }, 1000)

    /**
     * 点击body切换显示模式
     */
    $('body').on('click', () => {
        flag = !flag
        if (flag) {
            $('.clock').html(`
            <span class="hours"></span
            >&nbsp;:&nbsp;<span class="minutes"></span
            >
            `)
        } else {
            $('.clock').html(`
            <span class="hours"></span
            >&nbsp;:&nbsp;<span class="minutes"></span
            >&nbsp;:&nbsp;<span class="seconds"></span>
            `)
        }
    })

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
        const h = getHours   < 10 ? `0${getHours}`   : `${getHours}`
        const m = getMinutes < 10 ? `0${getMinutes}` : `${getMinutes}`
        const s = getSeconds < 10 ? `0${getSeconds}` : `${getSeconds}`
        /**
         * 渲染时间
         */
        $('.hours').text(h)
        $('.minutes').text(m)
        $('.seconds').text(s)
        $('.localeDate').text(dt.toLocaleDateString())
    }
})
