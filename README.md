# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2025 - Faculty of Computer Science, Universitas Indonesia

**Nama    : Ghina Nabila Gunawan**

**NPM     : 2206825914**

**Kelas   : AdvProg - A**

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:

    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |

2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create SubscriberRequest model struct.`
    -   [x] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [x] Commit: `Implement add function in Notification repository.`
    -   [x] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Commit: `Implement receive_notification function in Notification service.`
    -   [x] Commit: `Implement receive function in Notification controller.`
    -   [x] Commit: `Implement list_messages function in Notification service.`
    -   [x] Commit: `Implement list function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

### Reflection Subscriber-1️⃣
**1. In this tutorial, we used RwLock<> to synchronise the use of Vec of Notifications. Explain why it is necessary for this case, and explain why we do not use Mutex<> instead?**
> Dalam implementasi ini, kita menggunakan `RwLock<>` untuk **menghindari bottleneck** dalam **data access**. `RwLock<>` memungkinkan **banyak thread** untuk `read()` secara bersamaan, tapi hanya satu thread yang bisa `write()`.
Kalau kita menggunakan `Mutex<>`, hanya **satu thread** yang bisa mengakses data dalam satu waktu, baik untuk membaca maupun menulis. Hal ini akan memperlambat program kalau ada banyak pembacaan data karena semua thread harus menunggu giliran untuk mendapatkan akses, meskipun hanya untuk membaca. 
Pada case ini, kita lebih sering membaca daftar notifikasi daripada menulisnya. Dengan `RwLock<>`, banyak thread bisa membaca data **secara paralel**, dan hanya akan di-lock saat ada **proses `write()`**.

**2.  In this tutorial, we used lazy_static external library to define Vec and DashMap as a “static” variable. Compared to Java where we can mutate the content of a static variable via a static function, why did not Rust allow us to do so?**
> Di Rust, **variabel static bersifat immutable (tidak bisa diubah)** secara langsung karena Rust menerapkan konsep **memory safety** tanpa **garbage collector**. Kalau Rust mengizinkan direct modification terhadap variabel `static`, maka akan muncul **race condition**, di mana beberapa thread bisa mengakses dan mengubah data yang sama secara bersamaan tanpa kontrol yang jelas.
Sebagai solusi, kita menggunakan **`lazy_static!`** untuk membuat variabel `static` yang bisa berubah, tetapi tetap dengan kontrol akses menggunakan `RwLock<>` atau `Mutex<>`. Ini memastikan kalau semua mutasi tetap aman dan terkontrol.
Sedangkan di Java, kita bisa mengubah variabel `static` melalui method `static` karena Java menggunakan **garbage collector** yang bisa menangani **data race** dan **memory leaks** secara otomatis. Tapi Rust memberikan kita full control terhadap memory, jadi lebih ketat dalam menangani data modification secara paralel untuk mencegah bug yang sulit dideteksi.

---

### Reflection Subscriber-2️⃣
**1. Have you explored things outside the steps in the tutorial, for example: src/lib.rs?**
> Selama mengerjakan tugas ini, saya lebih fokus mengikuti step-step yang ada di tutorial untuk memastikan implementasi berjalan dengan baik. 
Jadi saya tidak terlalu mengeksplorasi `src/lib.rs` atau bagian lain yang tidak disebutkan secara langsung.

> Tapi, setelah memahami keseluruhan sistem, saya mencoba melihat struktur di `src/lib.rs`. Dari sana, saya menyadari kalau file ini berperan sebagai **entry point utama** yang mengatur bagaimana modul dalam aplikasi kita di-load. 
> Saya belajar kalau dalam proyek Rust berbasis **modular**, file `lib.rs` sering digunakan untuk mengatur ekspor dari berbagai modul yang ada. Ini berguna untuk menjaga struktur kode tetap rapi dan memudahkan pengelolaan dependensi antar-modul.

**2. Explain how Observer pattern eases you to plug in more subscribers. How about spawning more than one instance of Main app, will it still be easy enough to add to the system?**
> Setelah menyelesaikan tutorial dan menguji sistem notifikasi dengan menjalankan beberapa instance dari **Receiver**, saya menyadari kalau **Observer Pattern** sangat membantu dalam mempermudah proses menambahkan subscriber baru.
 Observer Pattern memungkinkan setiap subscriber untuk menerima notifikasi tanpa perlu mengubah main code. Ini sangat fleksibel karena kita bisa menambahkan subscriber baru tanpa harus mengutak-atik implementasi dari **NotificationService**.
 
> Kalau kita mau menjalankan beberapa instance dari main app (bukan hanya Receivers), tantangannya agak lebih kompleks. 
> Kita perlu memastikan kalau setiap instance tetap bisa menangani notifikasi dengan cara yang efisien, misalnya dengan menggunakan **message queue** seperti Kafka supaya event-driven architecture tetap berjalan dengan optimal.

**3. Have you tried to make your own Tests, or enhance documentation on your Postman collection?**
>  Saya mencoba menulis beberapa **unit test** sederhana untuk memastikan kalau fungsi `receive_notification` dan `list_message` bekerja sesuai harapan. Dari experience ini, saya menyadari kalau punya **test yang baik** sangat penting, terutama dalam event base system seperti ini.

> Selama proses ini, saya mencoba melakukan testing dengan cara menjalankan tiga instance Receiver yang masing-masing mendengarkan di port **8001, 8002, dan 8003**, serta satu instance dari aplikasi utama di port **8000**.
> 
>   - Dengan menggunakan file `.env`, saya mengatur **ROCKET_PORT** supaya setiap instance Receiver berjalan di port yang berbeda. 
>   - Saya juga mengubah **APP_INSTANCE_NAME** untuk membedakan setiap instance yang berjalan. 
>   - Setelah itu, saya menjalankan beberapa request melalui **Postman**, seperti: Subscribe ke produk tertentu dan Cek apakah instance menerima notifikasi

> Untuk memastikan **API terdokumentasi** dengan baik, saya mengimpor **Postman Collection JSON** yang disediakan pada link di atas. Setelah itu, saya melakukan beberapa penyesuaian dan pengisian template yang sudah ada:
> 
> - Saya menambahkan environment variables seperti `{{receiver_url}}` dan `{{publisher_url}}` untuk membuat request lebih fleksibel. 
> - Misalnya, kalau ada perubahan port dari **8001** ke **8002**, saya hanya perlu mengubah satu variabel tanpa perlu mengedit setiap request secara manual.

---