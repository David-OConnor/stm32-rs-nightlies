#[doc = "Register `BTR` reader"]
pub struct R(crate::R<BTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR` writer"]
pub struct W(crate::W<BTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR_SPEC>;
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
impl From<crate::W<BTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - BRP"]
pub type BRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRP` writer - BRP"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u16, u16, 10, O>;
#[doc = "Field `TS1` reader - TS1"]
pub type TS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1` writer - TS1"]
pub type TS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TS2` reader - TS2"]
pub type TS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS2` writer - TS2"]
pub type TS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SJW` reader - SJW"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - SJW"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LBKM` reader - LBKM"]
pub type LBKM_R = crate::BitReader<bool>;
#[doc = "Field `LBKM` writer - LBKM"]
pub type LBKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTR_SPEC, bool, O>;
#[doc = "Field `SILM` reader - SILM"]
pub type SILM_R = crate::BitReader<bool>;
#[doc = "Field `SILM` writer - SILM"]
pub type SILM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<0> {
        BRP_W::new(self)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<16> {
        TS1_W::new(self)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<20> {
        TS2_W::new(self)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<24> {
        SJW_W::new(self)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    #[must_use]
    pub fn lbkm(&mut self) -> LBKM_W<30> {
        LBKM_W::new(self)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    #[must_use]
    pub fn silm(&mut self) -> SILM_W<31> {
        SILM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_BTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](index.html) module"]
pub struct BTR_SPEC;
impl crate::RegisterSpec for BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr::R](R) reader structure"]
impl crate::Readable for BTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr::W](W) writer structure"]
impl crate::Writable for BTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTR to value 0"]
impl crate::Resettable for BTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
