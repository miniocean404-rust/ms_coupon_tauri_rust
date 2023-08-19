import { getEncrypt } from './encrypt'

export async function sign(fullUrl, data, cookie, ua) {
  const url = 'https://market.waimai.meituan.com'
  const referrer = 'https://passport.meituan.com/'

  getEncrypt(window)

  // 执行 加密 文件的加密函数
  data.mtFingerprint = window.H5guard.getfp()

  return await window.H5guard.sign({
    url: fullUrl,
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      cookie,
    },
    data,
  })
}
