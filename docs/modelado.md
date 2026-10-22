# Dominio del Problema

## Diagrama del Dominio
![Diagrama Modelado](img/Autoescuela.png)

## Entidades del Problema

### Alumno

Representa a los alumnos apuntados a las clases prácticas de la autoescuela. Cada uno tiene identidad propia, distinguidos por su DNI (es único para todas las personas). Cada uno de ellos
estarán asignados a una clase práctica (el profesor se encarga de eso), pudiendo cancelar la clase si al final no van a poder asistir. Sabiendo esto, se ha decidido modelar a los 
alumnos como una entidad, con atributos DNI (un String con 8 caracteres y una letra), así como el nombre (otro String, necesario para que el profesor pueda saber mejor a quién recoger)
y la dirección (un objeto valor, inmutable, que se detallará a continuación). 

### Clase

Representa las clases prácticas que imparte el profesor de la Autoescuela. Todas duran 45 minutos. Cada clase tiene que ir asignada a un alumno (se enlaza con el DNI del alumno), 
siendo este uno registrado en la Autoescuela. Una clase no puede empezar el mismo día a la misma hora que otra, por lo que la fecha sirve para diferenciarlas. Esto hace que se haya
decidido modelar las clases como una entidad. Los atributos, por tanto, son la hora de inicio (es tipo SystemTime, por lo que engloba fecha y hora, es en segundos y es el tiempo que ha
pasado desde el 1 de Enero de 1970, tiempo Unix), el DNI del alumno que va a asistir a la clase (tipo String) y un objeto valor inmutable que representa el estado de la clase (si 
está activa o se ha cancelado, incluyendo, si ha sido cancelada, el instante de tiempo en el que se canceló la clase).

## Objetos Valor del Problema

### Direccion

Representa la dirección donde vive un alumno, dato necesario por el profesor para saber dónde recogerlo. Al ser un atributo inmutable y sin identidad propia (si el alumno cambia de dirección 
simplemente se sustituye la dirección vieja por la nueva), compuesto por varios atributos, se ha decidido modelarlo como un objeto valor. Se compone por la calle (String), el número de 
la casa (String), o bien, si se trata de un piso, el número del piso (String, es un campo opcional).

### EstadoClase

Representa el estado actual de una clase. Como ya sabemos, un alumno puede decidir cancelar la clase, por lo que el sistema necesita registrar si la clase esta activa o cancelada, siendo 
necesario el instante de tiempo en el que se canceló para poder calcular el margen de tiempo con respecto al inicio de la siguiente clase (lógica de negocio). Al ser un objeto inmutable
(en el sentido de que no tiene identidad propia y si se cancela una clase se sustituye por un estado cancelado junto al instante de tiempo), se ha decidido modelarse como un objeto valor.
Como representa estados, se ha modelado como un Enum con dos posibles estados, Activa o Cancelada. Eso sí, si se ha cancelado, este objeto debe guardar el instante en el que se canceló, es 
obligatorio.

### Ubicación Coche

Representa la ubicación en la que se encuentra el profesor mientras está impartiendo las clases. Es una coordenada necesaria para poder calcular el tiempo que tarda en llegar a la dirección
de un alumno al que haya decidido elegir para la próxima clase (cuando otro le ha cancelado). No tiene identidad propia, por lo que se ha decidido modelar como un objeto valor. Se compone
por la latitud y la longitud (son coordenadas, tipo float) y el instante de actualización, es decir, como el profesor se va moviendo de un lado para otro, es necesario guardar el momento 
en el que se guardó la ubicación (para ir actualizándose).

## Agregados

### Alumnos Registrados

Engloba a todos los alumnos apuntados a las clases prácticas de la Autoescuela. Es la única fuente de verdad, por lo que se encarga de crear y guardar a los alumnos, además de comprobar que
no existan dos alumnos iguales (mismo DNI). Para conseguir esto, se modela como un diccionario cuya clave es el DNI del alumno, para asegurar que no se registren alumnos con el mismo DNI.

### Agenda

Engloba todas las clases prácticas que tiene que impartir el profesor. Es la única fuente de verdad, se encarga de la gestión de las clases. También comprueba que las clases no se solapen
(que respeten la duración entre clases, 45 minutos, así como que no puedan haber dos clases el mismo día a la misma hora). Se ha decidido modelar la Agenda como un BTreeMap, que ordena las
claves (que en este caso es la hora de inicio, la forma de identificar las clases) y permite gestionar rangos más fácil (para evitar solapamientos). Se comprueba también que el alumno 
asignado a la clase realmente pertenece a la Autoescuela (pertenece al agregado Alumnos Registrados). Por último, la Agenda también se encarga de almacenar la ubicación del coche del profesor,
puede verse como una agenda física en la que el profesor va apuntando su ubicación cada cierto tiempo.
