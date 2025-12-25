<img align="left" width="80" height="80" src="assets/icon.png">

# Hachimi-Edge

[English](README.md) | Español | [Tiếng Việt](README.vi.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md)

[![Discord server](https://dcbadge.limes.pink/api/server/https://discord.gg/BVEt5FcxEn)](https://discord.gg/BVEt5FcxEn)

Mod de mejora y traducción del juego para UM:PD.

<img height="400" src="assets/screenshot.jpg">

# ⚠️ Por favor no enlace a este repositorio o al sitio web de Hachimi-Edge
Entendemos que quieres ayudar a la gente a instalar Hachimi-Edge y tener una mejor experiencia jugando. Sin embargo, este proyecto va inherentemente en contra de los TOS del juego y el Desarrollador del Juego definitivamente quiere que desaparezca si llegaran a enterarse.

Aunque compartir en tus servicios de chat autogestionados y a través de mensajería privada está bien, te pedimos humildemente que te abstengas de compartir enlaces a este proyecto en sitios de cara al público, o a cualquiera de las herramientas involucradas.

O compártelos y arruínalo para las docenas de usuarios de Hachimi-Edge. Depende de ti.

### Si vas a compartirlo de todos modos
Haz lo que debas, pero respetuosamente te solicitamos que intentes etiquetar el juego como "UM:PD" o "The Honse Game" en lugar del nombre real del juego, para evitar el análisis de motores de búsqueda.

# Acerca de Este Fork

Este fork se centra en la **optimización para Windows y Linux (Proton)**. Construido sobre las mejoras DXVK de Mario0051 con enfoque adicional en:

- ✅ **Calidad del Código** - Refactorización masiva para linting adecuado de Rust (snake_case, corrección de advertencias)
- ✅ **Soporte Linux/Proton** - Compatibilidad DXVK mejorada y optimizaciones específicas para Proton
- ✅ **Solo Windows/Proton** - Base de código simplificada, soporte de Android eliminado para desarrollo enfocado

**Linaje del Repositorio:**
```
kairusds/Hachimi-Edge → Mario0051/Hachimi-Edge → tenshou170/Hachimi-Edge (Este Fork)
     (Repo principal)         (Soporte DXVK inicial)    (Proton mejorado + Calidad de código)
```

# Características
- **Traducciones de alta calidad:** Hachimi-Edge viene con funciones de traducción avanzadas que ayudan a que las traducciones se sientan más naturales (formas plurales, números ordinales, etc.) y evitan introducir problemas en la UI. También admite la traducción de la mayoría de los componentes del juego; ¡no se necesita parchear activos manualmente!

    Componentes compatibles:
    - Texto de UI
    - master.mdb (nombre de habilidad, descripción de habilidad, etc.)
    - Historia de carrera
    - Historia principal/Diálogo de inicio
    - Letras
    - Reemplazo de texturas
    - Reemplazo de atlas de sprites

    Además, Hachimi-Edge no proporciona funciones de traducción para un solo idioma; ha sido diseñado para ser completamente configurable para cualquier idioma.

- **Configuración sencilla:** Simplemente conecta y juega. Toda la configuración se realiza dentro del propio juego, ¡no se necesita ninguna aplicación externa!
- **Actualización automática de traducciones:** El actualizador de traducciones integrado permite jugar con normalidad mientras se actualiza, y lo recarga en el juego cuando termina, ¡no es necesario reiniciar!
- **GUI integrada:** Incluye un editor de configuración para poder modificar los ajustes sin salir del juego!
- **Configuración gráfica:** Puedes ajustar los parámetros gráficos del juego para aprovechar al máximo las especificaciones de tu dispositivo, incluyendo:
  - Desbloqueo de FPS
  - Escalado de resolución
  - Opciones de antialiasing (MSAA)
  - Control de sincronización vertical (VSync)
  - Opciones de modo pantalla completa
- **Soporte Linux/Proton:** Compatibilidad mejorada con DXVK y Proton para una experiencia de juego en Linux sin problemas.
- **Multiplataforma:** Diseñado desde cero para ser portable, con soporte para Windows y Linux (Proton).

# Instalación
Por favor, consulta la página [Getting started](https://hachimi.noccu.art/docs/hachimi/getting-started.html).

# Agradecimientos especiales
Estos proyectos han sido la base para el desarrollo de Hachimi-Edge; sin ellos, Hachimi-Edge nunca habría existido en su forma actual:

- [Trainers' Legend G](https://github.com/MinamiChiwa/Trainers-Legend-G)
- [umamusume-localify-android](https://github.com/Kimjio/umamusume-localify-android)
- [umamusume-localify](https://github.com/GEEKiDoS/umamusume-localify)
- [Carotenify](https://github.com/KevinVG207/Uma-Carotenify)
- [umamusu-translate](https://github.com/noccu/umamusu-translate)
- [frida-il2cpp-bridge](https://github.com/vfsfitvnm/frida-il2cpp-bridge)

**Créditos específicos del fork:**
- **Hachimi Original** - [LeadRDRK](https://github.com/LeadRDRK) y el Equipo Hachimi
- **Hachimi-Edge (Principal)** - [kairusds](https://github.com/kairusds)
- **Mejoras DXVK/Linux** - [Mario0051](https://github.com/Mario0051)

# Licencia
[GNU GPLv3](LICENSE)
