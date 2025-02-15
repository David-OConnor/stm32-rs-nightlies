#[doc = "Register `TR` reader"]
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR` writer"]
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SU` reader - SU"]
pub type SU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SU` writer - SU"]
pub type SU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ST` reader - ST"]
pub type ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST` writer - ST"]
pub type ST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MNU` reader - MNU"]
pub type MNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNU` writer - MNU"]
pub type MNU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MNT` reader - MNT"]
pub type MNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNT` writer - MNT"]
pub type MNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 3, O>;
#[doc = "Field `HU` reader - HU"]
pub type HU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HU` writer - HU"]
pub type HU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 4, O>;
#[doc = "Field `HT` reader - HT"]
pub type HT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HT` writer - HT"]
pub type HT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - PM"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - SU"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - MNU"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - MNT"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - HU"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - HT"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SU"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<0> {
        SU_W::new(self)
    }
    #[doc = "Bits 4:6 - ST"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<4> {
        ST_W::new(self)
    }
    #[doc = "Bits 8:11 - MNU"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<8> {
        MNU_W::new(self)
    }
    #[doc = "Bits 12:14 - MNT"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<12> {
        MNT_W::new(self)
    }
    #[doc = "Bits 16:19 - HU"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<16> {
        HU_W::new(self)
    }
    #[doc = "Bits 20:21 - HT"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<20> {
        HT_W::new(self)
    }
    #[doc = "Bit 22 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<22> {
        PM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](index.html) module"]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr::R](R) reader structure"]
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr::W](W) writer structure"]
impl crate::Writable for TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
