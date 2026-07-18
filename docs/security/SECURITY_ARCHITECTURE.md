# Mira Security Architecture

## Amaç

Bu belge, Mira'nın güvenlik mimarisini üst düzeyde tanımlar.

## Temel Bileşenler

### Kimlik Doğrulama

- Müdebbir kimliği güvenli token ile doğrulanır.
- Token yalnızca ortam değişkeni olarak kullanılır.
- Token kaynak kodunda tutulmaz.

### Yetkilendirme

- Kimliği doğrulanmamış istekler reddedilir.
- Kimliği doğrulanmış istekler yetkilerine göre işlenir.

### GitHub Güvenliği

- Repository Secrets kullanılır.
- GitHub Actions üzerinden canlı doğrulama yapılır.
- Başarısız doğrulamalar dağıtımı durdurmalıdır.

### Rasterast Doğrulaması

Rasterast aşağıdaki maddeleri doğrular:

- Kimlik doğrulama
- Yetkilendirme
- Secret güvenliği
- Tutarlılık
- Denetlenebilirlik

## Güvenlik İlkeleri

- En az yetki (Least Privilege)
- Varsayılan olarak reddet (Deny by Default)
- Gizli bilgileri koru
- Denetlenebilir işlem geçmişi
- Müdebbir onayı gerektiren kritik işlemler

## Sonuç

Bu mimari, Mira'nın güvenlik altyapısının temelini oluşturur ve üretim ortamındaki tüm güvenlik uygulamaları bu ilkelere uygun olmalıdır.

