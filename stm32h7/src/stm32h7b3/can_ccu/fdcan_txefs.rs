#[doc = "Register `FDCAN_TXEFS` reader"]
pub struct R(crate::R<FDCAN_TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXEFS` writer"]
pub struct W(crate::W<FDCAN_TXEFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXEFS_SPEC>;
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
impl From<crate::W<FDCAN_TXEFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXEFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFFL` reader - Event FIFO Fill Level"]
pub type EFFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFFL` writer - Event FIFO Fill Level"]
pub type EFFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXEFS_SPEC, u8, u8, 6, O>;
#[doc = "Field `EFGI` reader - Event FIFO Get Index."]
pub type EFGI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFGI` writer - Event FIFO Get Index."]
pub type EFGI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXEFS_SPEC, u8, u8, 5, O>;
#[doc = "Field `EFF` reader - Event FIFO Full."]
pub type EFF_R = crate::BitReader<bool>;
#[doc = "Field `EFF` writer - Event FIFO Full."]
pub type EFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TXEFS_SPEC, bool, O>;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost."]
pub type TEFL_R = crate::BitReader<bool>;
#[doc = "Field `TEFL` writer - Tx Event FIFO Element Lost."]
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TXEFS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn effl(&mut self) -> EFFL_W<0> {
        EFFL_W::new(self)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    #[must_use]
    pub fn efgi(&mut self) -> EFGI_W<8> {
        EFGI_W::new(self)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    #[must_use]
    pub fn eff(&mut self) -> EFF_W<24> {
        EFF_W::new(self)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<25> {
        TEFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txefs](index.html) module"]
pub struct FDCAN_TXEFS_SPEC;
impl crate::RegisterSpec for FDCAN_TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txefs::R](R) reader structure"]
impl crate::Readable for FDCAN_TXEFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txefs::W](W) writer structure"]
impl crate::Writable for FDCAN_TXEFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXEFS to value 0"]
impl crate::Resettable for FDCAN_TXEFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
