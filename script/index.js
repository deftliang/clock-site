$(document).ready(function () {
  // 显示模式开关
  let modeFlag = false
  // nav open
  let navOpen = false
  // localeDate mode open
  let localeDateOpen = false
  const $btn = $('#setting')
  const $path = $('#path')
  const $nav = $('.nav')
  const $localeDate = $('.localeDate')

  /**
   * 时钟运行
   */
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
    $('.localeDate').text(dt.toLocaleDateString())
  }

  // setting button
  $nav.on('click', function (e) {
    const key = e.target.getAttribute('key')
    switch (key) {
      case '0':
        if (localeDateOpen) {
          $localeDate.attr('style', 'display:none;')
          localeDateOpen = false
          e.target.innerText = 'Show Date'
        } else {
          $localeDate.attr('style', 'display:block;')
          localeDateOpen = true
          e.target.innerText = 'Hide Date'
        }
        break
      case '1':
        modeFlag = !modeFlag
        if (modeFlag) {
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
        break
      default:
        break
    }
    $nav.attr('style', 'display:none;')
    navOpen = false
    $path.attr('fill', '#535c68')
    $btn.attr('style', 'transform:rotate(0deg)')
    e.stopPropagation()
  })
  $btn.on('click', function (e) {
    if (navOpen) {
      $nav.attr('style', 'display:none;')
      navOpen = false
    } else {
      $nav.attr('style', 'display:block;')
      navOpen = true
    }
    e.stopPropagation()
  })
  $btn.on('mouseenter', function (e) {
    $path.attr('fill', '#f2f2f2')
    $btn.attr('style', 'transform:rotate(180deg)')
    e.stopPropagation()
  })
  $btn.on('mouseleave', function (e) {
    if (navOpen) return
    $path.attr('fill', '#535c68')
    $btn.attr('style', 'transform:rotate(0deg)')
    e.stopPropagation()
  })
  $('body').on('click', function (e) {
    if (navOpen) {
      if (e.target !== $nav.get(0) && e.target !== $btn.get(0)) {
        $nav.attr('style', 'display:none;')
        navOpen = false
        $path.attr('fill', '#535c68')
        $btn.attr('style', 'transform:rotate(0deg)')
      }
    }
    e.stopPropagation()
  })
})
