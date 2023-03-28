#[doc = "Register `AHB4RSTR` reader"]
pub struct R(crate::R<AHB4RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB4RSTR` writer"]
pub struct W(crate::W<AHB4RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4RSTR_SPEC>;
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
impl From<crate::W<AHB4RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOARST` reader - GPIOA block reset Set and reset by software."]
pub type GPIOARST_R = crate::BitReader<GPIOARST_A>;
#[doc = "GPIOA block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIOARST_A> {
        match self.bits {
            true => Some(GPIOARST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST_A::Reset
    }
}
#[doc = "Field `GPIOARST` writer - GPIOA block reset Set and reset by software."]
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4RSTR_SPEC, GPIOARST_A, O>;
impl<'a, const O: u8> GPIOARST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOARST_A::Reset)
    }
}
#[doc = "Field `GPIOBRST` reader - GPIOB block reset Set and reset by software."]
pub use GPIOARST_R as GPIOBRST_R;
#[doc = "Field `GPIOCRST` reader - GPIOC block reset Set and reset by software."]
pub use GPIOARST_R as GPIOCRST_R;
#[doc = "Field `GPIODRST` reader - GPIOD block reset Set and reset by software."]
pub use GPIOARST_R as GPIODRST_R;
#[doc = "Field `GPIOERST` reader - GPIOE block reset Set and reset by software."]
pub use GPIOARST_R as GPIOERST_R;
#[doc = "Field `GPIOFRST` reader - GPIOF block reset Set and reset by software."]
pub use GPIOARST_R as GPIOFRST_R;
#[doc = "Field `GPIOGRST` reader - GPIOG block reset Set and reset by software."]
pub use GPIOARST_R as GPIOGRST_R;
#[doc = "Field `GPIOHRST` reader - GPIOH block reset Set and reset by software."]
pub use GPIOARST_R as GPIOHRST_R;
#[doc = "Field `GPIOIRST` reader - GPIOI block reset Set and reset by software."]
pub use GPIOARST_R as GPIOIRST_R;
#[doc = "Field `GPIOJRST` reader - GPIOJ block reset Set and reset by software."]
pub use GPIOARST_R as GPIOJRST_R;
#[doc = "Field `GPIOKRST` reader - GPIOK block reset Set and reset by software."]
pub use GPIOARST_R as GPIOKRST_R;
#[doc = "Field `BDMA2RST` reader - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
pub use GPIOARST_R as BDMA2RST_R;
#[doc = "Field `GPIOBRST` writer - GPIOB block reset Set and reset by software."]
pub use GPIOARST_W as GPIOBRST_W;
#[doc = "Field `GPIOCRST` writer - GPIOC block reset Set and reset by software."]
pub use GPIOARST_W as GPIOCRST_W;
#[doc = "Field `GPIODRST` writer - GPIOD block reset Set and reset by software."]
pub use GPIOARST_W as GPIODRST_W;
#[doc = "Field `GPIOERST` writer - GPIOE block reset Set and reset by software."]
pub use GPIOARST_W as GPIOERST_W;
#[doc = "Field `GPIOFRST` writer - GPIOF block reset Set and reset by software."]
pub use GPIOARST_W as GPIOFRST_W;
#[doc = "Field `GPIOGRST` writer - GPIOG block reset Set and reset by software."]
pub use GPIOARST_W as GPIOGRST_W;
#[doc = "Field `GPIOHRST` writer - GPIOH block reset Set and reset by software."]
pub use GPIOARST_W as GPIOHRST_W;
#[doc = "Field `GPIOIRST` writer - GPIOI block reset Set and reset by software."]
pub use GPIOARST_W as GPIOIRST_W;
#[doc = "Field `GPIOJRST` writer - GPIOJ block reset Set and reset by software."]
pub use GPIOARST_W as GPIOJRST_W;
#[doc = "Field `GPIOKRST` writer - GPIOK block reset Set and reset by software."]
pub use GPIOARST_W as GPIOKRST_W;
#[doc = "Field `BDMA2RST` writer - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
pub use GPIOARST_W as BDMA2RST_W;
impl R {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOG block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOI block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOJ block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOK block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn bdma2rst(&self) -> BDMA2RST_R {
        BDMA2RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    #[doc = "Bit 4 - GPIOE block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    #[doc = "Bit 5 - GPIOF block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    #[doc = "Bit 6 - GPIOG block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    #[doc = "Bit 8 - GPIOI block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<8> {
        GPIOIRST_W::new(self)
    }
    #[doc = "Bit 9 - GPIOJ block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<9> {
        GPIOJRST_W::new(self)
    }
    #[doc = "Bit 10 - GPIOK block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<10> {
        GPIOKRST_W::new(self)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn bdma2rst(&mut self) -> BDMA2RST_W<21> {
        BDMA2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4rstr](index.html) module"]
pub struct AHB4RSTR_SPEC;
impl crate::RegisterSpec for AHB4RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb4rstr::R](R) reader structure"]
impl crate::Readable for AHB4RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb4rstr::W](W) writer structure"]
impl crate::Writable for AHB4RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB4RSTR to value 0"]
impl crate::Resettable for AHB4RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
