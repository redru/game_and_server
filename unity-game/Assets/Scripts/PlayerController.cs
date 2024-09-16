using UnityEngine;
using UnityEngine.InputSystem;

public class PlayerController : MonoBehaviour
{
    private static readonly int Moving = Animator.StringToHash("moving");
    private static readonly int DirY = Animator.StringToHash("dirY");
    private static readonly int DirX = Animator.StringToHash("dirX");
    private Animator _animator;
    private Rigidbody2D _rigidbody;
    private InputActions _input;

    private Vector2 _direction = Vector2.zero;

    private void Awake()
    {
        _animator = GetComponent<Animator>();
        _rigidbody = GetComponent<Rigidbody2D>();
        _input = new InputActions();
    }

    private void OnEnable()
    {
        _input.Enable();
        _input.Player.Move.performed += OnMovementPerformed;
        _input.Player.Move.canceled += OnMovementCancelled;
    }

    private void OnDisable()
    {
        _input.Disable();
        _input.Player.Move.performed -= OnMovementPerformed;
        _input.Player.Move.canceled -= OnMovementCancelled;
    }

    private void FixedUpdate()
    {
        _rigidbody.velocity = _direction * (250 * Time.fixedDeltaTime);
    }

    private void OnMovementPerformed(InputAction.CallbackContext value)
    {
        _direction = value.ReadValue<Vector2>();
        _animator.SetBool(Moving, true);
        _animator.SetFloat(DirX, _direction.x);
        _animator.SetFloat(DirY, _direction.y);
    }

    private void OnMovementCancelled(InputAction.CallbackContext value)
    {
        _direction = Vector2.zero;
        _animator.SetBool(Moving, false);
        _animator.SetFloat(DirX, 0);
        _animator.SetFloat(DirY, 0);
    }
}
