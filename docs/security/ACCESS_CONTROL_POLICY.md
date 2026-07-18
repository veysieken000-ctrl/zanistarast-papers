# Mira Access Control Policy

## Amaç

Bu belge, Mira sistemindeki erişim kontrol kurallarını tanımlar.

## Temel İlkeler

- Varsayılan politika "Deny by Default" olmalıdır.
- En az yetki (Least Privilege) uygulanmalıdır.
- Her istek kimlik doğrulamasından geçmelidir.
- Kritik işlemler Müdebbir onayı gerektirir.

## Roller

### Müdebbir

Yetkiler:

- Sistem yapılandırması
- Güvenlik politikaları
- Yayın onayı
- Secret yönetimi
- Acil durum işlemleri

### Mira

Yetkiler:

- Analiz
- Planlama
- Görev oluşturma
- Rasterast doğrulaması başlatma
- Tavsiye üretme

Kısıtlar:

- Yayın yapamaz.
- Secret değiştiremez.
- Müdebbir adına karar veremez.

### Rasterast

Yetkiler:

- Tutarlılık doğrulaması
- Güvenlik doğrulaması
- Risk analizi

Kısıtlar:

- Sistem yapılandırmasını değiştiremez.
- Yayın gerçekleştiremez.

## Yetkilendirme Kuralları

Aşağıdaki işlemler yalnızca Müdebbir tarafından gerçekleştirilebilir:

- Production dağıtımı
- Secret güncelleme
- Güvenlik politikası değişikliği
- Yayın işlemleri

## Sonuç

Bu politika, Mira güvenlik mimarisindeki rol tabanlı erişim kontrolünün temelini oluşturur.


