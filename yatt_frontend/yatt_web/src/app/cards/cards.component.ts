import { Component, OnInit } from '@angular/core';
import { DataApiService } from '../../services/data-api.service';
import { Card, CardWithId } from '../project.model';
import {FormsModule} from '@angular/forms';
import {NgForOf, NgIf} from '@angular/common';

@Component({
  selector: 'app-cards',
  templateUrl: './cards.component.html',
  styleUrls: ['./cards.component.css'],
  imports: [
    FormsModule,
    NgIf,
    NgForOf
  ]
})
export class CardsComponent implements OnInit {
  cards: CardWithId[] = [];
  cardForm: CardWithId = { card_name: '', tag_id: '' };
  showModal = false;
  isEdit = false;

  constructor(private dataApiService: DataApiService) {}

  ngOnInit(): void {
    this.fetchCards();
  }

  fetchCards(): void {
    this.dataApiService.getCards().subscribe((cards) => {
      this.cards = cards.cards;
    });
  }

  openAddModal(): void {
    this.isEdit = false;
    this.cardForm = { card_name: '', tag_id: this.generateTagId() };
    this.showModal = true;
  }

  openEditModal(card: CardWithId): void {
    this.isEdit = true;
    this.cardForm = { ...card };
    this.showModal = true;
  }

  closeModal(): void {
    this.showModal = false;
  }

  saveCard(): void {
    if (this.isEdit) {
      this.dataApiService.updateCard(this.cardForm.id!.id.String, this.cardForm).subscribe(() => {
        this.fetchCards();
        this.closeModal();
      });
    } else {
      this.dataApiService.addCard(this.cardForm).subscribe(() => {
        this.fetchCards();
        this.closeModal();
      });
    }
  }

  deleteCard(card: CardWithId): void {
    this.dataApiService.deleteCard(card.id!.id.String).subscribe(() => {
      this.fetchCards();
    });
  }

  generateTagId(): string {
    return "0";
  }
}
