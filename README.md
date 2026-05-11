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
![Connections 1](../assets/connections-1.png)

### 2. Log Penerimaan Pesan
Berikut adalah log di terminal saat Subscriber menerima pesan dari Publisher secara bertahap (dengan delay 1 detik):
![Terminal Logs](../assets/terminal-logs.png)

### 3. Simulasi Antrian (Queued Messages)
Saat Subscriber dibuat lambat dan Publisher mengirim pesan berkali-kali, pesan akan menumpuk di antrian:
![Queued Messages](../assets/queued-messages.png)

**Pertanyaan: Mengapa jumlah total antrian (queue) bisa mencapai angka tertentu?**
Jumlah antrian ditentukan oleh selisih antara kecepatan **Publisher** mengirim pesan dan kecepatan **Subscriber** memprosesnya. Karena kita menambahkan `thread::sleep` selama 1 detik pada Subscriber, maka setiap kali Publisher mengirim 5 pesan sekaligus, pesan-pesan tersebut akan mengantri di RabbitMQ sampai Subscriber selesai memproses satu per satu. Jika Publisher dijalankan berulang kali dengan cepat, jumlah antrian akan terus bertambah (misalnya mencapai 20 jika dijalankan 4 kali berturut-turut).
