Autonomous Fire Control System (FCS)

Python ve Rust tabanlı, düşük gecikmeli, gerçek zamanlı bir otonom hedef tespit ve takip sistemi prototipi.

🚀 Proje Hakkında

Bu proje, görsel verilerin yüksek hızlı işlenmesi ve bu verilerin otonom bir sistemle haberleştirilmesi üzerine kurulmuştur. YOLOv8 ile nesne tespiti, Kalman Filtresi ile hedef tahminleme ve Rust (Macroquad) ile geliştirilmiş düşük gecikmeli bir HUD (Head-Up Display) arayüzünü içerir.

🛠 Teknik Mimari

Backend (Python): YOLOv8 modelinin çalıştırılması, sensör verisi işleme ve UDP üzerinden veri paketleme.

Frontend (Rust): Düşük gecikme süresi için optimize edilmiş HUD arayüzü, UDP soket haberleşmesi.

Haberleşme: Gerçek zamanlı veri iletimi için UDP protokolü.

📋 Özellikler

[x] Anlık hedef tespiti ve sınıflandırma (YOLOv8).

[x] UDP üzerinden düşük gecikmeli veri akışı.

[x] Rust (Macroquad) tabanlı özelleştirilebilir HUD arayüzü.

[ ] Kalman Filtresi ile tahminleyici takip (Geliştiriliyor).

📈 Demo

Projenin çalışma görüntüsünü aşağıdan izleyebilirsiniz:
[Buraya YouTube veya LinkedIn videonun linkini yapıştır]

🚀 Kurulum

Backend (Python)

cd backend_sensor_fusion
pip install -r requirements.txt
python transmitter.py


Frontend (Rust)

cd frontend_hud
cargo run


📜 İletişim

Geliştirici: Esma Öztürk

LinkedIn: https://www.linkedin.com/in/esma-%C3%B6zt%C3%BCrk-5565aa373/

GitHub: https://github.com/Esmaozturk37/Autonomous-FCS-Prototype

Bu proje, otonom savunma sistemleri mimarisi üzerine bir araştırma ve geliştirme çalışmasıdır.