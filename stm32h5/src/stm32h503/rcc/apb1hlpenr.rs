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
#[doc = "Field `DTSLPEN` reader - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DTSLPEN` writer - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub type LPTIM2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub type LPTIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
#[doc = "Field `FDCAN1LPEN` reader - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
pub type FDCAN1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1LPEN` writer - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
pub type FDCAN1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, bool, O>;
impl R {
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
    #[doc = "Bit 9 - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn fdcan1lpen(&self) -> FDCAN1LPEN_R {
        FDCAN1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
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
    #[doc = "Bit 9 - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1lpen(&mut self) -> FDCAN1LPEN_W<9> {
        FDCAN1LPEN_W::new(self)
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
