#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub type EN1_R = crate::BitReader<EN1_A>;
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    #[doc = "0: DAC channel X disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X enabled"]
    Enabled = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::Disabled,
            true => EN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1_A::Enabled
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN1_A, O>;
impl<'a, const O: u8> EN1_W<'a, O> {
    #[doc = "DAC channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1_A::Disabled)
    }
    #[doc = "DAC channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1_A::Enabled)
    }
}
#[doc = "Field `BOFF1` reader - DAC channel1 output buffer disable"]
pub type BOFF1_R = crate::BitReader<BOFF1_A>;
#[doc = "DAC channel1 output buffer disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFF1_A {
    #[doc = "0: DAC channel X output buffer enabled"]
    Enabled = 0,
    #[doc = "1: DAC channel X output buffer disabled"]
    Disabled = 1,
}
impl From<BOFF1_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF1_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFF1_A {
        match self.bits {
            false => BOFF1_A::Enabled,
            true => BOFF1_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOFF1_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOFF1_A::Disabled
    }
}
#[doc = "Field `BOFF1` writer - DAC channel1 output buffer disable"]
pub type BOFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, BOFF1_A, O>;
impl<'a, const O: u8> BOFF1_W<'a, O> {
    #[doc = "DAC channel X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF1_A::Enabled)
    }
    #[doc = "DAC channel X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF1_A::Disabled)
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type TEN1_R = crate::BitReader<TEN1_A>;
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1_A {
    #[doc = "0: DAC channel X trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X trigger enabled"]
    Enabled = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::Disabled,
            true => TEN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1_A::Enabled
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type TEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEN1_A, O>;
impl<'a, const O: u8> TEN1_W<'a, O> {
    #[doc = "DAC channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1_A::Disabled)
    }
    #[doc = "DAC channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1_A::Enabled)
    }
}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub type TSEL1_R = crate::FieldReader<u8, TSEL1_A>;
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: Timer 6 TRGO event"]
    Tim6Trgo = 0,
    #[doc = "1: Timer 3 TRGO event"]
    Tim3Trgo = 1,
    #[doc = "2: Timer 7 TRGO event"]
    Tim7Trgo = 2,
    #[doc = "3: Timer 15 TRGO event"]
    Tim15Trgo = 3,
    #[doc = "4: Timer 2 TRGO event"]
    Tim2Trgo = 4,
    #[doc = "6: EXTI line9"]
    Exti9 = 6,
    #[doc = "7: Software trigger"]
    Software = 7,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEL1_A> {
        match self.bits {
            0 => Some(TSEL1_A::Tim6Trgo),
            1 => Some(TSEL1_A::Tim3Trgo),
            2 => Some(TSEL1_A::Tim7Trgo),
            3 => Some(TSEL1_A::Tim15Trgo),
            4 => Some(TSEL1_A::Tim2Trgo),
            6 => Some(TSEL1_A::Exti9),
            7 => Some(TSEL1_A::Software),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tim6Trgo`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL1_A::Tim6Trgo
    }
    #[doc = "Checks if the value of the field is `Tim3Trgo`"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == TSEL1_A::Tim3Trgo
    }
    #[doc = "Checks if the value of the field is `Tim7Trgo`"]
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL1_A::Tim7Trgo
    }
    #[doc = "Checks if the value of the field is `Tim15Trgo`"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == TSEL1_A::Tim15Trgo
    }
    #[doc = "Checks if the value of the field is `Tim2Trgo`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1_A::Tim2Trgo
    }
    #[doc = "Checks if the value of the field is `Exti9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1_A::Exti9
    }
    #[doc = "Checks if the value of the field is `Software`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TSEL1_A::Software
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub type TSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, TSEL1_A, 3, O>;
impl<'a, const O: u8> TSEL1_W<'a, O> {
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim6Trgo)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim3Trgo)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim7Trgo)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim15Trgo)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim2Trgo)
    }
    #[doc = "EXTI line9"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1_A::Exti9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL1_A::Software)
    }
}
#[doc = "Field `WAVE2` reader - WAVE2"]
pub type WAVE2_R = crate::BitReader<bool>;
#[doc = "Field `WAVE2` writer - WAVE2"]
pub type WAVE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::BitReader<bool>;
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MAMP10` reader - MAMP10"]
pub type MAMP10_R = crate::BitReader<bool>;
#[doc = "Field `MAMP10` writer - MAMP10"]
pub type MAMP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MAMP11` reader - MAMP11"]
pub type MAMP11_R = crate::BitReader<bool>;
#[doc = "Field `MAMP11` writer - MAMP11"]
pub type MAMP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MAMP12` reader - MAMP12"]
pub type MAMP12_R = crate::BitReader<bool>;
#[doc = "Field `MAMP12` writer - MAMP12"]
pub type MAMP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MAMP13` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP13_R = crate::BitReader<bool>;
#[doc = "Field `MAMP13` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub type DMAEN1_R = crate::BitReader<DMAEN1_A>;
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1_A {
    #[doc = "0: DAC channel X DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X DMA mode enabled"]
    Enabled = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::Disabled,
            true => DMAEN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1_A::Enabled
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub type DMAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAEN1_A, O>;
impl<'a, const O: u8> DMAEN1_W<'a, O> {
    #[doc = "DAC channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Disabled)
    }
    #[doc = "DAC channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Enabled)
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_R = crate::BitReader<DMAUDRIE1_A>;
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1_A {
    #[doc = "0: DAC channel X DMA Underrun Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X DMA Underrun Interrupt enabled"]
    Enabled = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAUDRIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::Disabled,
            true => DMAUDRIE1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1_A::Enabled
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAUDRIE1_A, O>;
impl<'a, const O: u8> DMAUDRIE1_W<'a, O> {
    #[doc = "DAC channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::Disabled)
    }
    #[doc = "DAC channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - WAVE2"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MAMP10"]
    #[inline(always)]
    pub fn mamp10(&self) -> MAMP10_R {
        MAMP10_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MAMP11"]
    #[inline(always)]
    pub fn mamp11(&self) -> MAMP11_R {
        MAMP11_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MAMP12"]
    #[inline(always)]
    pub fn mamp12(&self) -> MAMP12_R {
        MAMP12_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp13(&self) -> MAMP13_R {
        MAMP13_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn boff1(&mut self) -> BOFF1_W<1> {
        BOFF1_W::new(self)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<2> {
        TEN1_W::new(self)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<3> {
        TSEL1_W::new(self)
    }
    #[doc = "Bit 6 - WAVE2"]
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<6> {
        WAVE2_W::new(self)
    }
    #[doc = "Bit 7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<7> {
        WAVE1_W::new(self)
    }
    #[doc = "Bit 8 - MAMP10"]
    #[inline(always)]
    #[must_use]
    pub fn mamp10(&mut self) -> MAMP10_W<8> {
        MAMP10_W::new(self)
    }
    #[doc = "Bit 9 - MAMP11"]
    #[inline(always)]
    #[must_use]
    pub fn mamp11(&mut self) -> MAMP11_W<9> {
        MAMP11_W::new(self)
    }
    #[doc = "Bit 10 - MAMP12"]
    #[inline(always)]
    #[must_use]
    pub fn mamp12(&mut self) -> MAMP12_W<10> {
        MAMP12_W::new(self)
    }
    #[doc = "Bit 11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp13(&mut self) -> MAMP13_W<11> {
        MAMP13_W::new(self)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<12> {
        DMAEN1_W::new(self)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<13> {
        DMAUDRIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
