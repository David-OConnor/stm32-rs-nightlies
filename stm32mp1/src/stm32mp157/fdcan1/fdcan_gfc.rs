#[doc = "Register `FDCAN_GFC` reader"]
pub struct R(crate::R<FDCAN_GFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_GFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_GFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_GFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_GFC` writer"]
pub struct W(crate::W<FDCAN_GFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_GFC_SPEC>;
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
impl From<crate::W<FDCAN_GFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_GFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRFE` reader - RRFE"]
pub type RRFE_R = crate::BitReader<bool>;
#[doc = "Field `RRFE` writer - RRFE"]
pub type RRFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_GFC_SPEC, bool, O>;
#[doc = "Field `RRFS` reader - RRFS"]
pub type RRFS_R = crate::BitReader<bool>;
#[doc = "Field `RRFS` writer - RRFS"]
pub type RRFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_GFC_SPEC, bool, O>;
#[doc = "Field `ANFE` reader - ANFE"]
pub type ANFE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANFE` writer - ANFE"]
pub type ANFE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_GFC_SPEC, u8, u8, 2, O>;
#[doc = "Field `ANFS` reader - ANFS"]
pub type ANFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANFS` writer - ANFS"]
pub type ANFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_GFC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<0> {
        RRFE_W::new(self)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<1> {
        RRFS_W::new(self)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<2> {
        ANFE_W::new(self)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<4> {
        ANFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_gfc](index.html) module"]
pub struct FDCAN_GFC_SPEC;
impl crate::RegisterSpec for FDCAN_GFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_gfc::R](R) reader structure"]
impl crate::Readable for FDCAN_GFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_gfc::W](W) writer structure"]
impl crate::Writable for FDCAN_GFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_GFC to value 0"]
impl crate::Resettable for FDCAN_GFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
