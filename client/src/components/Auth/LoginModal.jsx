import {
  Modal,
  Button,
  Group,
  TextInput,
  Text,
  PasswordInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useState } from "react";

export default function LoginModal({ opened, close, login }) {
  const [loading, setLoading] = useState(false);
  const [login_error, set_loginError] = useState(false);
  const form = useForm({
    initialValues: {
      username: "",
      password: "",
    },
  });

  return (
    <Modal opened={opened} onClose={close} title="Login to Account">
      <form
        onSubmit={form.onSubmit(async (values) => {
          //Call Login here, toggle the submit button as disabled.
          setLoading(true);
          set_loginError(false);
          //Perform Login here. On success close modal, on failure, show error.
          let result = await login(values.username, values.password);
          if (result.success) {
            //Login worked. Close modal, button should be changed.
            close();
            setLoading(false);
          } else {
            console.log(
              `Modal: Got error logging in here, probably server error handle correctly.`
            );
            set_loginError(true);
            setLoading(false);
          }
        })}
      >
        <TextInput
          withAsterisk
          label="Username"
          placeholder="your username"
          {...form.getInputProps("username")}
        />
        <PasswordInput
          withAsterisk
          label="Password"
          placeholder="Enter your Password...."
          {...form.getInputProps("password")}
        />
        {login_error ? (
          <Text c="red" fw={400} size="sm">
            There was an error connecting to Auth server.
          </Text>
        ) : null}
        <Group position="right" mt="md">
          {loading ? (
            <Button loading color="orange">
              Logging in
            </Button>
          ) : (
            <Button type="submit">Submit</Button>
          )}
        </Group>
      </form>
    </Modal>
  );
}
