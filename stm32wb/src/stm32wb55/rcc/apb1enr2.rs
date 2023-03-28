#[doc = "Register `APB1ENR2` reader"]
pub struct R(crate::R<APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR2` writer"]
pub struct W(crate::W<APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR2_SPEC>;
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
impl From<crate::W<APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1EN` reader - CPU1 Low power UART 1 clock enable"]
pub type LPUART1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1EN` writer - CPU1 Low power UART 1 clock enable"]
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR2_SPEC, bool, O>;
#[doc = "Field `LPTIM2EN` reader - CPU1 LPTIM2EN"]
pub type LPTIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2EN` writer - CPU1 LPTIM2EN"]
pub type LPTIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU1 Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - CPU1 LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 Low power UART 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<0> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 5 - CPU1 LPTIM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<5> {
        LPTIM2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr2](index.html) module"]
pub struct APB1ENR2_SPEC;
impl crate::RegisterSpec for APB1ENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr2::R](R) reader structure"]
impl crate::Readable for APB1ENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr2::W](W) writer structure"]
impl crate::Writable for APB1ENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for APB1ENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
