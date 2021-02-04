type Init = {
  cmd: "init";
};

type Log = {
  cmd: "log";
  text: string;
};

type Cmd = Init | Log;

const invoke = <T extends Cmd>(cmd: T) => {
  try {
    window.external["invoke"](JSON.stringify(cmd));
  } catch (err) {
    console.error(err);
  }
};

const init = () => {
  invoke<Init>({ cmd: "init" });
};

const log = (text: string) => {
  invoke<Log>({ cmd: "log", text });
};

export const rpc = {
  init,
  log,
};
