#[doc = "Register `BCR2` reader"]
pub struct R(crate::R<BCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCR2` writer"]
pub struct W(crate::W<BCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR2_SPEC>;
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
impl From<crate::W<BCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MBKEN` reader - MBKEN"]
pub type MBKEN_R = crate::BitReader<bool>;
#[doc = "Field `MBKEN` writer - MBKEN"]
pub type MBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `MUXEN` reader - MUXEN"]
pub type MUXEN_R = crate::BitReader<bool>;
#[doc = "Field `MUXEN` writer - MUXEN"]
pub type MUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `MTYP` reader - MTYP"]
pub type MTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTYP` writer - MTYP"]
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `MWID` reader - MWID"]
pub type MWID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWID` writer - MWID"]
pub type MWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `FACCEN` reader - FACCEN"]
pub type FACCEN_R = crate::BitReader<bool>;
#[doc = "Field `FACCEN` writer - FACCEN"]
pub type FACCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `BURSTEN` reader - BURSTEN"]
pub type BURSTEN_R = crate::BitReader<bool>;
#[doc = "Field `BURSTEN` writer - BURSTEN"]
pub type BURSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `WAITPOL` reader - WAITPOL"]
pub type WAITPOL_R = crate::BitReader<bool>;
#[doc = "Field `WAITPOL` writer - WAITPOL"]
pub type WAITPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `WAITCFG` reader - WAITCFG"]
pub type WAITCFG_R = crate::BitReader<bool>;
#[doc = "Field `WAITCFG` writer - WAITCFG"]
pub type WAITCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `WREN` reader - WREN"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - WREN"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `WAITEN` reader - WAITEN"]
pub type WAITEN_R = crate::BitReader<bool>;
#[doc = "Field `WAITEN` writer - WAITEN"]
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `EXTMOD` reader - EXTMOD"]
pub type EXTMOD_R = crate::BitReader<bool>;
#[doc = "Field `EXTMOD` writer - EXTMOD"]
pub type EXTMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `ASYNCWAIT` reader - ASYNCWAIT"]
pub type ASYNCWAIT_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCWAIT` writer - ASYNCWAIT"]
pub type ASYNCWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `CPSIZE` reader - CRAM page size"]
pub type CPSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPSIZE` writer - CRAM page size"]
pub type CPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CBURSTRW` reader - CBURSTRW"]
pub type CBURSTRW_R = crate::BitReader<bool>;
#[doc = "Field `CBURSTRW` writer - CBURSTRW"]
pub type CBURSTRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR2_SPEC, bool, O>;
#[doc = "Field `NBLSET` reader - Byte lane (NBL) setup"]
pub type NBLSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBLSET` writer - Byte lane (NBL) setup"]
pub type NBLSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Byte lane (NBL) setup"]
    #[inline(always)]
    pub fn nblset(&self) -> NBLSET_R {
        NBLSET_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    #[must_use]
    pub fn mbken(&mut self) -> MBKEN_W<0> {
        MBKEN_W::new(self)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<1> {
        MUXEN_W::new(self)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<2> {
        MTYP_W::new(self)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<4> {
        MWID_W::new(self)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    #[must_use]
    pub fn faccen(&mut self) -> FACCEN_W<6> {
        FACCEN_W::new(self)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn bursten(&mut self) -> BURSTEN_W<8> {
        BURSTEN_W::new(self)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    #[must_use]
    pub fn waitpol(&mut self) -> WAITPOL_W<9> {
        WAITPOL_W::new(self)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    #[must_use]
    pub fn waitcfg(&mut self) -> WAITCFG_W<11> {
        WAITCFG_W::new(self)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<12> {
        WREN_W::new(self)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<13> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    #[must_use]
    pub fn extmod(&mut self) -> EXTMOD_W<14> {
        EXTMOD_W::new(self)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<15> {
        ASYNCWAIT_W::new(self)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    #[must_use]
    pub fn cpsize(&mut self) -> CPSIZE_W<16> {
        CPSIZE_W::new(self)
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    #[must_use]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<19> {
        CBURSTRW_W::new(self)
    }
    #[doc = "Bits 22:23 - Byte lane (NBL) setup"]
    #[inline(always)]
    #[must_use]
    pub fn nblset(&mut self) -> NBLSET_W<22> {
        NBLSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR-Flash chip-select control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr2](index.html) module"]
pub struct BCR2_SPEC;
impl crate::RegisterSpec for BCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcr2::R](R) reader structure"]
impl crate::Readable for BCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcr2::W](W) writer structure"]
impl crate::Writable for BCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR2 to value 0x30d0"]
impl crate::Resettable for BCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x30d0;
}
