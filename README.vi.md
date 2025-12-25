<img align="left" width="80" height="80" src="assets/icon.png">

# Hachimi-Edge

[English](README.md) | [Español](README.es.md) | Tiếng Việt | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md)

[![Discord server](https://dcbadge.limes.pink/api/server/https://discord.gg/BVEt5FcxEn)](https://discord.gg/BVEt5FcxEn)

Mod cải tiến và dịch game cho UM:PD.

<img height="400" src="assets/screenshot.jpg">

# ⚠️ Vui lòng không link đến repo này hoặc trang web của Hachimi-Edge
Chúng tôi hiểu rằng bạn muốn giúp mọi người cài đặt Hachimi-Edge và có trải nghiệm chơi game tốt hơn. Tuy nhiên, dự án này về bản chất đi ngược lại TOS của game và Nhà Phát Triển Game chắc chắn muốn nó biến mất nếu họ biết về nó.

Mặc dù chia sẻ trong các dịch vụ chat tự quản lý và thông qua tin nhắn riêng tư là được, chúng tôi khiêm tốn yêu cầu bạn không chia sẻ link đến dự án này trên các trang web công khai, hoặc đến bất kỳ công cụ nào liên quan.

Hoặc chia sẻ chúng và phá hỏng nó cho hàng chục người dùng Hachimi-Edge. Tùy bạn.

### Nếu bạn vẫn định chia sẻ
Làm những gì bạn phải làm, nhưng chúng tôi xin kính cẩn yêu cầu bạn cố gắng gọi game là "UM:PD" hoặc "The Honse Game" thay vì tên thực của game, để tránh phân tích cú pháp của công cụ tìm kiếm.

# Về Fork Này

Fork này tập trung vào **tối ưu hóa Windows và Linux (Proton)**. Xây dựng dựa trên các cải tiến DXVK của Mario0051 với trọng tâm bổ sung vào:

- ✅ **Chất Lượng Code** - Tái cấu trúc toàn diện cho Rust linting đúng chuẩn (snake_case, sửa warnings)
- ✅ **Hỗ Trợ Linux/Proton** - Tương thích DXVK nâng cao và tối ưu hóa đặc biệt cho Proton
- ✅ **Chỉ Windows/Proton** - Codebase được tối ưu, loại bỏ hỗ trợ Android để phát triển tập trung

**Dòng Dõi Repository:**
```
kairusds/Hachimi-Edge → Mario0051/Hachimi-Edge → tenshou170/Hachimi-Edge (Fork Này)
     (Repo chính)            (Hỗ trợ DXVK ban đầu)     (Proton nâng cao + Chất lượng code)
```

# Tính Năng
- **Bản dịch chất lượng cao:** Hachimi-Edge đi kèm với các tính năng dịch nâng cao giúp bản dịch cảm thấy tự nhiên hơn (dạng số nhiều, số thứ tự, v.v.) và ngăn chặn việc gây lỗi cho UI. Nó cũng hỗ trợ dịch hầu hết các thành phần trong game; không cần vá thủ công assets!

    Thành phần được hỗ trợ:
    - Text UI
    - master.mdb (tên skill, mô tả skill, v.v.)
    - Câu chuyện đua
    - Câu chuyện chính/Hội thoại trang chủ
    - Lời bài hát
    - Thay thế texture  
    - Thay thế atlas sprite

    Ngoài ra, Hachimi-Edge không chỉ cung cấp tính năng dịch cho một ngôn ngữ duy nhất; nó được thiết kế để có thể cấu hình hoàn toàn cho bất kỳ ngôn ngữ nào.

- **Thiết lập dễ dàng:** Chỉ cần cắm và chơi. Tất cả thiết lập được thực hiện trong chính game, không cần ứng dụng bên ngoài!
- **Tự động cập nhật bản dịch:** Trình cập nhật dịch tích hợp cho phép bạn chơi game bình thường trong khi nó cập nhật, và tải lại trong game khi hoàn thành, không cần khởi động lại!
- **GUI tích hợp:** Đi kèm với trình chỉnh sửa cấu hình để bạn có thể sửa đổi cài đặt mà không cần thoát khỏi game!
- **Cài đặt đồ họa:** Bạn có thể điều chỉnh cài đặt đồ họa của game để tận dụng tối đa thông số thiết bị của mình, bao gồm:
  - Mở khóa FPS
  - Scaling độ phân giải
  - Tùy chọn anti-aliasing (MSAA)
  - Điều khiển VSync
  - Tùy chọn chế độ fullscreen
- **Hỗ trợ Linux/Proton:** Tương thích nâng cao với DXVK và Proton cho trải nghiệm chơi game Linux mượt mà.
- **Đa nền tảng:** Được thiết kế ngay từ đầu để có thể portable, với hỗ trợ Windows và Linux (Proton).

# Cài Đặt
Vui lòng xem trang [Getting started](https://hachimi.noccu.art/docs/hachimi/getting-started.html).

# Cảm ơn đặc biệt
Những dự án này là nền tảng cho sự phát triển của Hachimi-Edge; không có chúng, Hachimi-Edge sẽ không bao giờ tồn tại ở dạng hiện tại:

- [Trainers' Legend G](https://github.com/MinamiChiwa/Trainers-Legend-G)
- [umamusume-localify-android](https://github.com/Kimjio/umamusume-localify-android)
- [umamusume-localify](https://github.com/GEEKiDoS/umamusume-localify)
- [Carotenify](https://github.com/KevinVG207/Uma-Carotenify)
- [umamusu-translate](https://github.com/noccu/umamusu-translate)
- [frida-il2cpp-bridge](https://github.com/vfsfitvnm/frida-il2cpp-bridge)

**Credits đặc biệt cho fork:**
- **Hachimi Gốc** - [LeadRDRK](https://github.com/LeadRDRK) và Nhóm Hachimi
- **Hachimi-Edge (Chính)** - [kairusds](https://github.com/kairusds)
- **Cải Tiến DXVK/Linux** - [Mario0051](https://github.com/Mario0051)

# Giấy Phép
[GNU GPLv3](LICENSE)
