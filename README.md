# Module-9-Software-Architectures-Subscriber

## Pertanyaan Tutorial

### a. Apa gunanya protokol AMQP?
AMQP (Advanced Message Queuing Protocol) adalah protokol standar terbuka untuk pengiriman pesan antar aplikasi atau organisasi. Gunanya adalah untuk memastikan pesan terkirim dengan aman, efisien, dan dapat diandalkan meskipun sistem pengirim dan penerima menggunakan platform yang berbeda.

### b. Apa arti dari "amqp://guest:guest@localhost:5672"?
Ini adalah URL koneksi ke broker pesan (RabbitMQ):
- **`amqp://`**: Skema protokol yang digunakan.
- **`guest:guest`**: Kredensial login default (Username:Password).
- **`localhost:5672`**: Hostname (`localhost`) dan Port (`5672`) tempat RabbitMQ berjalan.

## Dokumentasi Pengujian

### 1. Koneksi Berhasil
Berikut adalah tampilan dashboard RabbitMQ saat program Subscriber telah terhubung:
![Connections 1](./assets/connections-1.png)

### 2. Log Penerimaan Pesan
Berikut adalah log di terminal saat Subscriber menerima pesan dari Publisher secara bertahap (dengan delay 1 detik):
![Terminal Logs](./assets/terminal-logs.png)

### 3. Simulasi Antrian (Queued Messages)
Saat Subscriber dibuat lambat dan Publisher mengirim pesan berkali-kali, pesan akan menumpuk di antrian:
![Queued Messages](./assets/queued-messages.png)

**Pertanyaan: Mengapa jumlah total antrian (queue) bisa mencapai angka tertentu?**
Jumlah antrian ditentukan oleh selisih antara kecepatan **Publisher** mengirim pesan dan kecepatan **Subscriber** memprosesnya. Karena kita menambahkan `thread::sleep` selama 1 detik pada Subscriber, maka setiap kali Publisher mengirim 5 pesan sekaligus, pesan-pesan tersebut akan mengantri di RabbitMQ sampai Subscriber selesai memproses satu per satu. Jika Publisher dijalankan berulang kali dengan cepat, jumlah antrian akan terus bertambah (misalnya mencapai 20 jika dijalankan 4 kali berturut-turut).

## Eksperimen: Multiple Subscribers (Competing Consumers)

Untuk mempercepat pemrosesan, kita menjalankan 3 instance Subscriber sekaligus.

### 1. Pembagian Beban di Terminal
Berikut adalah tampilan 3 terminal Subscriber yang bekerja bersama-sama. Terlihat bahwa pesan-pesan disebar secara merata ke ketiga subscriber tersebut:
![Competing Consumers](./assets/competing-consumers.png)

### 2. Grafik RabbitMQ (3 Consumers)
Dapat dilihat bahwa sekarang terdapat 3 Consumers yang terhubung ke broker:
![Multiple Subscribers Graph](./assets/multiple-subscribers-graph.png)

### Refleksi
**Mengapa lonjakan antrian (queue) berkurang jauh lebih cepat daripada sebelumnya?**
Hal ini terjadi karena kita menerapkan pola **Competing Consumers**. Dengan adanya 3 Subscriber yang berjalan secara paralel, RabbitMQ akan membagi pesan-pesan yang masuk ke dalam antrian secara bergantian (*Round Robin*) kepada setiap subscriber yang tersedia. Karena beban kerja dibagi ke 3 "petugas", maka total waktu yang dibutuhkan untuk mengosongkan antrian menjadi 3x lebih cepat dibandingkan hanya menggunakan 1 subscriber saja. Ini membuktikan bahwa arsitektur *Event-Driven* sangat mudah untuk di-*scale-out* guna menangani beban kerja yang tinggi.
