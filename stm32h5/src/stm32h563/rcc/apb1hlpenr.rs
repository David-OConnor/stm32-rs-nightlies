#[doc = "Register `APB1HLPENR` reader"]
pub struct R(crate::R<APB1HLPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HLPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HLPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HLPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HLPENR` writer"]
pub struct W(crate::W<APB1HLPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HLPENR_SPEC>;
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
impl From<crate::W<APB1HLPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HLPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART9LPEN` reader - UART9 clock enable during sleep mode Set and reset by software."]
pub type UART9LPEN_R = crate::BitReader<bool>;
#[doc = "Field `UART9LPEN` writer - UART9 clock enable during sleep mode Set and reset by software."]
pub type UART9LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
#[doc = "Field `UART12LPEN` reader - UART12 clock enable during sleep mode Set and reset by software."]
pub type UART12LPEN_R = crate::BitReader<bool>;
#[doc = "Field `UART12LPEN` writer - UART12 clock enable during sleep mode Set and reset by software."]
pub type UART12LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
#[doc = "Field `DTSLPEN` reader - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DTSLPEN` writer - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub type LPTIM2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub type LPTIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
#[doc = "Field `FDCAN12LPEN` reader - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
pub type FDCAN12LPEN_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN12LPEN` writer - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
pub type FDCAN12LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
#[doc = "Field `UCPDLPEN` reader - UCPD clock enable during sleep mode Set and reset by software."]
pub type UCPDLPEN_R = crate::BitReader<bool>;
#[doc = "Field `UCPDLPEN` writer - UCPD clock enable during sleep mode Set and reset by software."]
pub type UCPDLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART9 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart9lpen(&self) -> UART9LPEN_R {
        UART9LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart12lpen(&self) -> UART12LPEN_R {
        UART12LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DTS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn fdcan12lpen(&self) -> FDCAN12LPEN_R {
        FDCAN12LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23 - UCPD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ucpdlpen(&self) -> UCPDLPEN_R {
        UCPDLPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART9 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart9lpen(&mut self) -> UART9LPEN_W<0> {
        UART9LPEN_W::new(self)
    }
    #[doc = "Bit 1 - UART12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart12lpen(&mut self) -> UART12LPEN_W<1> {
        UART12LPEN_W::new(self)
    }
    #[doc = "Bit 3 - DTS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<3> {
        DTSLPEN_W::new(self)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<5> {
        LPTIM2LPEN_W::new(self)
    }
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcan12lpen(&mut self) -> FDCAN12LPEN_W<9> {
        FDCAN12LPEN_W::new(self)
    }
    #[doc = "Bit 23 - UCPD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpdlpen(&mut self) -> UCPDLPEN_W<23> {
        UCPDLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hlpenr](index.html) module"]
pub struct APB1HLPENR_SPEC;
impl crate::RegisterSpec for APB1HLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1hlpenr::R](R) reader structure"]
impl crate::Readable for APB1HLPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1hlpenr::W](W) writer structure"]
impl crate::Writable for APB1HLPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1HLPENR to value 0x4080_022b"]
impl crate::Resettable for APB1HLPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4080_022b;
}
