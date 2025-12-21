# Configuración para Railway

## Variables de Entorno Requeridas

### Obligatorias

1. **RPC_HTTP_URL**
   - URL del nodo RPC de la blockchain
   - Ejemplo: `http://216.106.182.100:32774`

2. **ES_URL**
   - URL de tu instancia de Elasticsearch en Railway
   - Si usas Railway Elasticsearch, obtén la URL de las variables de entorno del servicio
   - Ejemplo: `https://tu-elasticsearch.railway.app:9200` o `http://localhost:9200` si está en el mismo proyecto

### Opcionales (con valores por defecto)

3. **ES_USERNAME** (opcional)
   - Usuario de Elasticsearch si requiere autenticación
   - Ejemplo: `elastic`

4. **ES_PASSWORD** (opcional)
   - Contraseña de Elasticsearch
   - Ejemplo: `tu_password_secreto`

5. **INDEX_PREFIX** (default: `workqueue`)
   - Prefijo para los índices de Elasticsearch
   - Creará índices: `workqueue-blocks` y `workqueue-meta`

6. **BATCH_SIZE** (default: `1000`)
   - Número de bloques a procesar por lote
   - Valores más altos = más rápido pero más uso de memoria

7. **START_BLOCK** (default: `0`)
   - Bloque desde el cual empezar a indexar
   - `0` = desde el genesis block
   - Útil para reanudar desde un bloque específico

8. **SYNC_INTERVAL_SECS** (default: `2`)
   - Segundos entre verificaciones en modo live sync
   - Valores más bajos = más frecuente pero más carga en RPC

9. **CONCURRENCY** (default: `10`)
   - Número de tareas concurrentes para procesar bloques
   - Valores más altos = más rápido pero más carga en RPC

10. **ES_BULK_SIZE** (default: `100`)
    - Número de bloques a indexar en una operación bulk
    - Valores más altos = más rápido pero más uso de memoria

## Pasos para Desplegar

1. **Crea un nuevo proyecto en Railway**
   - Ve a [railway.app](https://railway.app)
   - Crea un nuevo proyecto
   - Conecta tu repositorio de GitHub

2. **Agrega el servicio de Elasticsearch (si no lo tienes)**
   - En Railway, agrega un nuevo servicio
   - Selecciona "Elasticsearch" del marketplace
   - Railway configurará automáticamente las variables de entorno

3. **Configura las Variables de Entorno**
   - En el dashboard de Railway, ve a tu servicio del indexer
   - Ve a la pestaña "Variables"
   - Agrega todas las variables de entorno listadas arriba
   - **IMPORTANTE**: No incluyas comillas en los valores

4. **Variables de Referencia de Railway (si Elasticsearch está en el mismo proyecto)**
   - Si Elasticsearch está en el mismo proyecto Railway, puedes usar:
   - `ES_URL` = URL pública que Railway asigna a tu servicio de Elasticsearch
   - O la URL interna si Railway la expone: `http://elasticsearch:9200`
   - Verifica en las variables de entorno del servicio de Elasticsearch

5. **Despliega**
   - Railway detectará automáticamente que es un proyecto Rust
   - El build se ejecutará automáticamente con `cargo build --release`
   - El servicio iniciará con `./target/release/blockchain-indexer`
   - El indexer correrá continuamente

## Comportamiento del Indexer

El indexer funciona en dos fases:

1. **Historical Sync**: Indexa todos los bloques desde `START_BLOCK` hasta el bloque actual
   - Procesa bloques en lotes de `BATCH_SIZE`
   - Guarda checkpoints después de cada lote
   - Muestra progreso, velocidad y ETA en los logs

2. **Live Sync**: Monitorea continuamente nuevos bloques y los indexa automáticamente
   - Verifica cada `SYNC_INTERVAL_SECS` segundos si hay nuevos bloques
   - Indexa nuevos bloques automáticamente
   - Corre indefinidamente

El indexer guarda checkpoints en Elasticsearch, por lo que si se reinicia, continuará desde el último bloque indexado.

## Verificación

Una vez desplegado, puedes verificar que funciona:

1. **Revisa los logs en Railway**
   - Deberías ver mensajes como:
     - "Initializing Blockchain Indexer..."
     - "Starting historical sync..."
     - "Processing batch: blocks X to Y"
     - "Live sync completed: now at block X"

2. **Verifica en Elasticsearch que se están creando los índices:**
   - `workqueue-blocks` (o `{INDEX_PREFIX}-blocks`)
   - `workqueue-meta` (o `{INDEX_PREFIX}-meta`)

3. **Consulta los datos indexados:**
   ```bash
   # Verificar que hay bloques indexados
   curl -X GET "ES_URL/workqueue-blocks/_count"
   
   # Ver el último checkpoint
   curl -X GET "ES_URL/workqueue-meta/_doc/checkpoint"
   ```

## Troubleshooting

**Error de conexión a Elasticsearch:**
- Verifica que `ES_URL` sea correcta
- Si Elasticsearch está en Railway, asegúrate de usar la URL correcta (pública o interna)
- Verifica las credenciales si están configuradas
- Asegúrate de que Elasticsearch esté accesible desde el servicio del indexer

**Error de conexión a RPC:**
- Verifica que `RPC_HTTP_URL` sea accesible desde Railway
- Algunos RPCs pueden tener restricciones de IP
- Verifica que el RPC esté funcionando correctamente

**El indexer se reinicia constantemente:**
- Revisa los logs para ver el error específico
- Verifica que todas las variables de entorno requeridas estén configuradas correctamente
- Railway tiene `restartPolicyType: ON_FAILURE` configurado, así que se reiniciará automáticamente si falla

**El indexer no avanza:**
- Verifica que el RPC esté respondiendo correctamente
- Revisa los logs para ver si hay errores de rate limiting
- Considera aumentar `SYNC_INTERVAL_SECS` si el RPC tiene límites

**Memoria insuficiente:**
- Reduce `BATCH_SIZE` o `ES_BULK_SIZE`
- Reduce `CONCURRENCY`
- Railway puede necesitar un plan con más recursos

## Notas Importantes

- El indexer corre **continuamente** en Railway
- Se reiniciará automáticamente si falla (hasta 10 intentos)
- Los checkpoints permiten reanudar desde donde se quedó
- El modo live sync mantiene la base de datos actualizada en tiempo real
- Railway puede pausar servicios inactivos en planes gratuitos, pero el indexer está activo constantemente

