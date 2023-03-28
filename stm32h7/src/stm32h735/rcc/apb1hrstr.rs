#[doc = "Register `APB1HRSTR` reader"]
pub struct R(crate::R<APB1HRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HRSTR` writer"]
pub struct W(crate::W<APB1HRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HRSTR_SPEC>;
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
impl From<crate::W<APB1HRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSRST` reader - Clock Recovery System reset"]
pub type CRSRST_R = crate::BitReader<CRSRST_A>;
#[doc = "Clock Recovery System reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<CRSRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRSRST_A> {
        match self.bits {
            true => Some(CRSRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRSRST_A::Reset
    }
}
#[doc = "Field `CRSRST` writer - Clock Recovery System reset"]
pub type CRSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HRSTR_SPEC, CRSRST_A, O>;
impl<'a, const O: u8> CRSRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::Reset)
    }
}
#[doc = "Field `SWPMIRST` reader - SWPMI block reset"]
pub use CRSRST_R as SWPMIRST_R;
#[doc = "Field `OPAMPRST` reader - OPAMP block reset"]
pub use CRSRST_R as OPAMPRST_R;
#[doc = "Field `MDIOSRST` reader - MDIOS block reset"]
pub use CRSRST_R as MDIOSRST_R;
#[doc = "Field `FDCANRST` reader - FDCAN block reset"]
pub use CRSRST_R as FDCANRST_R;
#[doc = "Field `TIM23RST` reader - TIM23 block reset"]
pub use CRSRST_R as TIM23RST_R;
#[doc = "Field `TIM24RST` reader - TIM24 block reset"]
pub use CRSRST_R as TIM24RST_R;
#[doc = "Field `SWPMIRST` writer - SWPMI block reset"]
pub use CRSRST_W as SWPMIRST_W;
#[doc = "Field `OPAMPRST` writer - OPAMP block reset"]
pub use CRSRST_W as OPAMPRST_W;
#[doc = "Field `MDIOSRST` writer - MDIOS block reset"]
pub use CRSRST_W as MDIOSRST_W;
#[doc = "Field `FDCANRST` writer - FDCAN block reset"]
pub use CRSRST_W as FDCANRST_W;
#[doc = "Field `TIM23RST` writer - TIM23 block reset"]
pub use CRSRST_W as TIM23RST_W;
#[doc = "Field `TIM24RST` writer - TIM24 block reset"]
pub use CRSRST_W as TIM24RST_W;
impl R {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    pub fn swpmirst(&self) -> SWPMIRST_R {
        SWPMIRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - TIM23 block reset"]
    #[inline(always)]
    pub fn tim23rst(&self) -> TIM23RST_R {
        TIM23RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TIM24 block reset"]
    #[inline(always)]
    pub fn tim24rst(&self) -> TIM24RST_R {
        TIM24RST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<1> {
        CRSRST_W::new(self)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    #[must_use]
    pub fn swpmirst(&mut self) -> SWPMIRST_W<2> {
        SWPMIRST_W::new(self)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<4> {
        OPAMPRST_W::new(self)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    #[must_use]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<5> {
        MDIOSRST_W::new(self)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<8> {
        FDCANRST_W::new(self)
    }
    #[doc = "Bit 24 - TIM23 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim23rst(&mut self) -> TIM23RST_W<24> {
        TIM23RST_W::new(self)
    }
    #[doc = "Bit 25 - TIM24 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim24rst(&mut self) -> TIM24RST_W<25> {
        TIM24RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hrstr](index.html) module"]
pub struct APB1HRSTR_SPEC;
impl crate::RegisterSpec for APB1HRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1hrstr::R](R) reader structure"]
impl crate::Readable for APB1HRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1hrstr::W](W) writer structure"]
impl crate::Writable for APB1HRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1HRSTR to value 0"]
impl crate::Resettable for APB1HRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
