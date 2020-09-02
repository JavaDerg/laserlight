import * as laser from 'laserlight/src_ts/index';

(async () => {
    await laser.init();
    laser.fancyUpDocument();
    laser.createGameCanvas();
})();
