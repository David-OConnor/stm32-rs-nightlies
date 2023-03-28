#[doc = "Register `APB1HENR` reader"]
pub struct R(crate::R<APB1HENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HENR` writer"]
pub struct W(crate::W<APB1HENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HENR_SPEC>;
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
impl From<crate::W<APB1HENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART9EN` reader - UART9 clock enable Set and reset by software."]
pub type UART9EN_R = crate::BitReader<bool>;
#[doc = "Field `UART9EN` writer - UART9 clock enable Set and reset by software."]
pub type UART9EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HENR_SPEC, bool, O>;
#[doc = "Field `UART12EN` reader - UART12 clock enable Set and reset by software."]
pub type UART12EN_R = crate::BitReader<bool>;
#[doc = "Field `UART12EN` writer - UART12 clock enable Set and reset by software."]
pub type UART12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HENR_SPEC, bool, O>;
#[doc = "Field `DTSEN` reader - DTS clock enable Set and reset by software."]
pub type DTSEN_R = crate::BitReader<bool>;
#[doc = "Field `DTSEN` writer - DTS clock enable Set and reset by software."]
pub type DTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HENR_SPEC, bool, O>;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 clock enable Set and reset by software."]
pub type LPTIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 clock enable Set and reset by software."]
pub type LPTIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HENR_SPEC, bool, O>;
#[doc = "Field `FDCAN12EN` reader - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
pub type FDCAN12EN_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN12EN` writer - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
pub type FDCAN12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HENR_SPEC, bool, O>;
#[doc = "Field `UCPDEN` reader - UCPD clock enable Set and reset by software."]
pub type UCPDEN_R = crate::BitReader<bool>;
#[doc = "Field `UCPDEN` writer - UCPD clock enable Set and reset by software."]
pub type UCPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART9 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart9en(&self) -> UART9EN_R {
        UART9EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART12 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart12en(&self) -> UART12EN_R {
        UART12EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DTS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn fdcan12en(&self) -> FDCAN12EN_R {
        FDCAN12EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23 - UCPD clock enable Set and reset by software."]
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART9 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart9en(&mut self) -> UART9EN_W<0> {
        UART9EN_W::new(self)
    }
    #[doc = "Bit 1 - UART12 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart12en(&mut self) -> UART12EN_W<1> {
        UART12EN_W::new(self)
    }
    #[doc = "Bit 3 - DTS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dtsen(&mut self) -> DTSEN_W<3> {
        DTSEN_W::new(self)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<5> {
        LPTIM2EN_W::new(self)
    }
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcan12en(&mut self) -> FDCAN12EN_W<9> {
        FDCAN12EN_W::new(self)
    }
    #[doc = "Bit 23 - UCPD clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpden(&mut self) -> UCPDEN_W<23> {
        UCPDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1henr](index.html) module"]
pub struct APB1HENR_SPEC;
impl crate::RegisterSpec for APB1HENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1henr::R](R) reader structure"]
impl crate::Readable for APB1HENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1henr::W](W) writer structure"]
impl crate::Writable for APB1HENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1HENR to value 0"]
impl crate::Resettable for APB1HENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
