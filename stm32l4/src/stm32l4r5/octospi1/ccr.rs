#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMODE` reader - Instruction mode"]
pub type IMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMODE` writer - Instruction mode"]
pub type IMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IDTR` reader - Instruction double transfer rate"]
pub type IDTR_R = crate::BitReader<bool>;
#[doc = "Field `IDTR` writer - Instruction double transfer rate"]
pub type IDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `ISIZE` reader - Instruction size"]
pub type ISIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISIZE` writer - Instruction size"]
pub type ISIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADMODE` reader - Address mode"]
pub type ADMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMODE` writer - Address mode"]
pub type ADMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADDTR` reader - Address double transfer rate"]
pub type ADDTR_R = crate::BitReader<bool>;
#[doc = "Field `ADDTR` writer - Address double transfer rate"]
pub type ADDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `ADSIZE` reader - Address size"]
pub type ADSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADSIZE` writer - Address size"]
pub type ADSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ABMODE` reader - Alternate byte mode"]
pub type ABMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABMODE` writer - Alternate byte mode"]
pub type ABMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ABDTR` reader - Alternate bytes double transfer rate"]
pub type ABDTR_R = crate::BitReader<bool>;
#[doc = "Field `ABDTR` writer - Alternate bytes double transfer rate"]
pub type ABDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `ABSIZE` reader - Alternate bytes size"]
pub type ABSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABSIZE` writer - Alternate bytes size"]
pub type ABSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMODE` reader - Data mode"]
pub type DMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMODE` writer - Data mode"]
pub type DMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DDTR` reader - Alternate bytes double transfer rate"]
pub type DDTR_R = crate::BitReader<bool>;
#[doc = "Field `DDTR` writer - Alternate bytes double transfer rate"]
pub type DDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `DQSE` reader - DQS enable"]
pub type DQSE_R = crate::BitReader<bool>;
#[doc = "Field `DQSE` writer - DQS enable"]
pub type DQSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `SIOO` reader - Send instruction only once mode"]
pub type SIOO_R = crate::BitReader<bool>;
#[doc = "Field `SIOO` writer - Send instruction only once mode"]
pub type SIOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Instruction mode"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Instruction double transfer rate"]
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Instruction size"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Address mode"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Address double transfer rate"]
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Alternate byte mode"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Alternate bytes double transfer rate"]
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Alternate bytes size"]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Data mode"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Alternate bytes double transfer rate"]
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DQS enable"]
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Send instruction only once mode"]
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Instruction mode"]
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<0> {
        IMODE_W::new(self)
    }
    #[doc = "Bit 3 - Instruction double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn idtr(&mut self) -> IDTR_W<3> {
        IDTR_W::new(self)
    }
    #[doc = "Bits 4:5 - Instruction size"]
    #[inline(always)]
    #[must_use]
    pub fn isize(&mut self) -> ISIZE_W<4> {
        ISIZE_W::new(self)
    }
    #[doc = "Bits 8:10 - Address mode"]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<8> {
        ADMODE_W::new(self)
    }
    #[doc = "Bit 11 - Address double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn addtr(&mut self) -> ADDTR_W<11> {
        ADDTR_W::new(self)
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<12> {
        ADSIZE_W::new(self)
    }
    #[doc = "Bits 16:18 - Alternate byte mode"]
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<16> {
        ABMODE_W::new(self)
    }
    #[doc = "Bit 19 - Alternate bytes double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn abdtr(&mut self) -> ABDTR_W<19> {
        ABDTR_W::new(self)
    }
    #[doc = "Bits 20:21 - Alternate bytes size"]
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<20> {
        ABSIZE_W::new(self)
    }
    #[doc = "Bits 24:26 - Data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<24> {
        DMODE_W::new(self)
    }
    #[doc = "Bit 27 - Alternate bytes double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn ddtr(&mut self) -> DDTR_W<27> {
        DDTR_W::new(self)
    }
    #[doc = "Bit 29 - DQS enable"]
    #[inline(always)]
    #[must_use]
    pub fn dqse(&mut self) -> DQSE_W<29> {
        DQSE_W::new(self)
    }
    #[doc = "Bit 31 - Send instruction only once mode"]
    #[inline(always)]
    #[must_use]
    pub fn sioo(&mut self) -> SIOO_W<31> {
        SIOO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
