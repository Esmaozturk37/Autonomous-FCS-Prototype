import cv2
from ultralytics import YOLO
import json
import socket
import numpy as np
from filterpy.kalman import KalmanFilter

# --- Kalman Filtresi Kurulumu ---
def get_kalman_filter():
    kf = KalmanFilter(dim_x=4, dim_z=2)
    # F: Hareket modeli (Konum + Hız)
    kf.F = np.array([[1, 0, 1, 0], [0, 1, 0, 1], [0, 0, 1, 0], [0, 0, 0, 1]])
    # H: Ölçüm modeli
    kf.H = np.array([[1, 0, 0, 0], [0, 1, 0, 0]])
    kf.P *= 1000.  # Kovaryans matrisi
    kf.R = 5       # Ölçüm gürültüsü
    kf.Q = 0.01    # Süreç gürültüsü
    return kf

model = YOLO('yolov8n.pt')
cap = cv2.VideoCapture(0)
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
server_address = ("127.0.0.1", 9999)

trackers = {} 

print("Sistem Aktif: Kalman + Öngörücü Takip + Ayna Modu Aktif...")

while cap.isOpened():
    ret, frame = cap.read()
    if not ret: break
    
    # --- GÖRÜNTÜYÜ YANSITMA (Aynalama) ---
    # Kameradaki terslik sorununu çözer, operatörün kendi hareket yönüyle uyumlu olur.
    frame = cv2.flip(frame, 1)
    
    results = model.track(frame, persist=True, verbose=False)
    
    targets = []
    if results[0].boxes.id is not None:
        boxes = results[0].boxes.xywh.cpu().numpy()
        ids = results[0].boxes.id.int().cpu().numpy()
        
        for box, obj_id in zip(boxes, ids):
            if obj_id not in trackers:
                trackers[obj_id] = get_kalman_filter()
                trackers[obj_id].x[:2] = np.array([[box[0]], [box[1]]])
            
            # 1. TAHMİN ET
            trackers[obj_id].predict()
            
            # 2. GÜNCELLE
            trackers[obj_id].update(np.array([[box[0]], [box[1]]]))
            
            # Tahmin edilen değerler
            x_pred = float(trackers[obj_id].x[0, 0])
            y_pred = float(trackers[obj_id].x[1, 0])
            vx = float(trackers[obj_id].x[2, 0])
            vy = float(trackers[obj_id].x[3, 0])
            
            # JSON paketine hız verilerini ekle
            targets.append({
                "id": int(obj_id),
                "mesafe": float(1500 / (box[3] + 1)),
                "vx": vx, 
                "vy": vy,
                "bbox": {
                    "x": float(x_pred/640), 
                    "y": float(y_pred/480), 
                    "w": float(box[2]/640), 
                    "h": float(box[3]/480)
                }
            })

    # Paketi gönder
    sock.sendto(json.dumps({"targets": targets}).encode(), server_address)
    
    # Görselleştirme (Yansıtılmış kareyi göster)
    cv2.imshow("C2-SENSOR (Predictive Tracking)", results[0].plot())
    if cv2.waitKey(1) & 0xFF == ord('q'): break

cap.release()
cv2.destroyAllWindows()