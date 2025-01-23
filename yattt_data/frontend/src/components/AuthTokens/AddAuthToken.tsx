import {
  Button,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Input,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
} from "@chakra-ui/react"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { type SubmitHandler, useForm } from "react-hook-form"

import { type ApiError, type AuthTokenCreate, AuthTokensService } from "../../client"
import useCustomToast from "../../hooks/useCustomToast"
import { handleError } from "../../utils"

interface AddItemProps {
  isOpen: boolean
  onClose: () => void
}

const AddAuthToken = ({ isOpen, onClose }: AddItemProps) => {
  const queryClient = useQueryClient()
  const showToast = useCustomToast()
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<AuthTokenCreate>({
    mode: "onBlur",
    criteriaMode: "all",
    defaultValues: {
      tag_id: '',
      device_id: '',
      description: '',
    },
  })

  const mutation = useMutation({
    mutationFn: (data: AuthTokenCreate) =>
      AuthTokensService.createAuthToken({ requestBody: data }),
    onSuccess: () => {
      showToast("Success!", "'AuthToken' created successfully.", "success")
      reset()
      onClose()
    },
    onError: (err: ApiError) => {
      handleError(err, showToast)
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["auth-tokens"] })
    },
  })

  const onSubmit: SubmitHandler<AuthTokenCreate> = (data) => {
    mutation.mutate(data)
  }

  return (
    <>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        size={{ base: "sm", md: "md" }}
        isCentered
      >
        <ModalOverlay />
        <ModalContent as="form" onSubmit={handleSubmit(onSubmit)}>
          <ModalHeader>Add Item</ModalHeader>
          <ModalCloseButton />
          <ModalBody pb={6}>
            <FormControl isRequired isInvalid={!!errors.tag_id}>
              <FormLabel htmlFor="tag_id">TagId</FormLabel>
              <Input
                  id="tag_id"
                  {...register('tag_id', {
                    required: 'tag_id is required.',
                  })}
                  placeholder="tag_id"
                  type="text"
              />
              {errors.tag_id && (
                  <FormErrorMessage>{errors.tag_id.message}</FormErrorMessage>
              )}
            </FormControl>
            <FormControl isRequired isInvalid={!!errors.device_id}>
              <FormLabel htmlFor="device_id">DeviceId</FormLabel>
              <Input
                  id="device_id"
                  {...register('device_id', {
                    required: 'device_id is required.',
                  })}
                  placeholder="device_id"
                  type="text"
              />
              {errors.device_id && (
                  <FormErrorMessage>{errors.device_id.message}</FormErrorMessage>
              )}
            </FormControl>
            <FormControl mt={4}>
              <FormLabel htmlFor="description">Description</FormLabel>
              <Input
                  id="description"
                  {...register('description')}
                  placeholder="Description"
                  type="text"
              />
            </FormControl>
          </ModalBody>

          <ModalFooter gap={3}>
            <Button variant="primary" type="submit" isLoading={isSubmitting}>
              Save
            </Button>
            <Button onClick={onClose}>Cancel</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}

export default AddAuthToken
