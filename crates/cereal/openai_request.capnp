@0xbcd0e2d4e021fab5;

struct OpenAIRequest {
  model @0 :Text;
  messages @1 :List(Message);
  maxTokens @2 :UInt16;
}

struct Message {
  role @0 :Text;
  content @1 :Text;
}

struct OpenAIResponse {
  choices @0 :List(Choice);
}

struct Choice {
  message @0 :MessageResponse;
}

struct MessageResponse {
  content @0 :Text;
}
