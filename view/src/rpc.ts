type Init = {
  cmd: "init";
};

type Log = {
  cmd: "log";
  text: string;
};

type Cmd = Init | Log;

const invoke = <T extends Cmd>(cmd: T) => {
  //@ts-ignore
  window.external.invoke(JSON.stringify(cmd));
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
