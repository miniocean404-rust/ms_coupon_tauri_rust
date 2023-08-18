import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './styles.css'
import { invoke } from '@tauri-apps/api/tauri'

// DOM 内容加载完成之后，通过 invoke 调用 在 Rust 中已经注册的命令
window.addEventListener('DOMContentLoaded', async () => {
  await invoke('close_splashscreen')
})

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
