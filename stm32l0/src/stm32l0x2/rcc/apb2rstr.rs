#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGRST` reader - System configuration controller reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRSTW_A>;
#[doc = "System configuration controller reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRSTW_A {
    #[doc = "1: Reset the module"]
    Reset = 1,
}
impl From<SYSCFGRSTW_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCFGRSTW_A> {
        match self.bits {
            true => Some(SYSCFGRSTW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRSTW_A::Reset
    }
}
#[doc = "Field `SYSCFGRST` writer - System configuration controller reset"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, SYSCFGRSTW_A, O>;
impl<'a, const O: u8> SYSCFGRST_W<'a, O> {
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRSTW_A::Reset)
    }
}
#[doc = "Field `TIM21RST` reader - TIM21 timer reset"]
pub use SYSCFGRST_R as TIM21RST_R;
#[doc = "Field `TIM22RST` reader - TIM22 timer reset"]
pub use SYSCFGRST_R as TIM22RST_R;
#[doc = "Field `ADCRST` reader - ADC interface reset"]
pub use SYSCFGRST_R as ADCRST_R;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub use SYSCFGRST_R as SPI1RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use SYSCFGRST_R as USART1RST_R;
#[doc = "Field `DBGRST` reader - DBG reset"]
pub use SYSCFGRST_R as DBGRST_R;
#[doc = "Field `TIM21RST` writer - TIM21 timer reset"]
pub use SYSCFGRST_W as TIM21RST_W;
#[doc = "Field `TIM22RST` writer - TIM22 timer reset"]
pub use SYSCFGRST_W as TIM22RST_W;
#[doc = "Field `ADCRST` writer - ADC interface reset"]
pub use SYSCFGRST_W as ADCRST_W;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub use SYSCFGRST_W as SPI1RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use SYSCFGRST_W as USART1RST_W;
#[doc = "Field `DBGRST` writer - DBG reset"]
pub use SYSCFGRST_W as DBGRST_W;
impl R {
    #[doc = "Bit 0 - System configuration controller reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM21 timer reset"]
    #[inline(always)]
    pub fn tim21rst(&self) -> TIM21RST_R {
        TIM21RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM22 timer reset"]
    #[inline(always)]
    pub fn tim22rst(&self) -> TIM22RST_R {
        TIM22RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - DBG reset"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 2 - TIM21 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim21rst(&mut self) -> TIM21RST_W<2> {
        TIM21RST_W::new(self)
    }
    #[doc = "Bit 5 - TIM22 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim22rst(&mut self) -> TIM22RST_W<5> {
        TIM22RST_W::new(self)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<9> {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 22 - DBG reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<22> {
        DBGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
